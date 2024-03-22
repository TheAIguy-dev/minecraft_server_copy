use std::{collections::VecDeque, fmt::Display};

use fastnbt::Value;
use log::debug;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::server::types::{Chunk, InteractionType, WriteString};
use crate::server::{
    state::ConnectionState,
    types::{
        self, Angle, EntityMetadata, Gamemode, PlayerActions, Position, ReadString, ReadVarInt,
        Uuid, WriteVarInt,
    },
};

#[allow(dead_code)]
pub enum IncomingPacket {
    Unknown {
        data: Vec<u8>,
    },
    /// Packet ID: 0x00
    Handshake {
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: i32,
    },
    // /// Packet ID: 0xFE
    // LegacyServerListPing {},
    /// Packet ID: 0x00
    StatusRequest,
    /// Packet ID: 0x01
    PingRequest {
        payload: i64,
    },
    /// Packet ID: 0x00
    LoginStart {
        name: String,
        player_uuid: Option<u128>,
    },
    /// Packet ID: 0x10
    Interact {
        entity_id: i32,
        interaction_type: InteractionType,
        target_pos: Option<(f32, f32, f32)>,
        hand: Option<i32>,
        sneaking: bool,
    },
    /// Packet ID: 0x12
    KeepAlive {
        keep_alive_id: i64,
    },
    /// Packet ID: 0x14
    SetPlayerPosition {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
    },
    /// Packet ID: 0x15
    SetPlayerPositionAndRotation {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    /// Packet ID: 0x16
    SetPlayerRotation {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    /// Packet ID: 0x17
    SetPlayerOnGround {
        on_ground: bool,
    },
}

impl Display for IncomingPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use IncomingPacket::*;
        write!(
            f,
            "{}",
            match self {
                Unknown { .. } => "Unknown",
                Handshake { .. } => "Handshake",
                StatusRequest => "StatusRequest",
                PingRequest { .. } => "PingRequest",
                LoginStart { .. } => "LoginStart",
                Interact { .. } => "Interact",
                KeepAlive { .. } => "KeepAlive",
                SetPlayerPosition { .. } => "SetPlayerPosition",
                SetPlayerPositionAndRotation { .. } => "SetPlayerPositionAndRotation",
                SetPlayerRotation { .. } => "SetPlayerRotation",
                SetPlayerOnGround { .. } => "SetPlayerOnGround",
            }
        )
    }
}

pub trait ReadPacket: AsyncRead + Unpin + Sized {
    async fn read_packet(&mut self, state: &ConnectionState) -> IncomingPacket {
        let length: i32 = self.read_varint().await;
        if length == 0 {
            return IncomingPacket::Unknown { data: vec![] };
        }
        let id: i32 = self.read_varint().await;

        use IncomingPacket::*;
        match (state, id) {
            (ConnectionState::Handshake, 0x00) => Handshake {
                protocol_version: self.read_varint().await,
                server_address: self.read_string().await,
                server_port: self.read_u16().await.unwrap_or_default(),
                next_state: self.read_varint().await,
            },
            (ConnectionState::Status, 0x00) => StatusRequest,
            (ConnectionState::Status, 0x01) => PingRequest {
                payload: self.read_i64().await.unwrap_or_default(),
            },
            (ConnectionState::Login, 0x00) => {
                let name: String = self.read_string().await;
                let has_player_uuid: bool = self.read_u8().await.unwrap_or_default() != 0;
                LoginStart {
                    name,
                    player_uuid: match has_player_uuid {
                        true => Some(self.read_u128().await.unwrap_or_default()),
                        false => None,
                    },
                }
            }
            (ConnectionState::Play, 0x10) => {
                debug!("Interact");
                let entity_id: i32 = self.read_varint().await;
                let interaction_type: InteractionType = match self.read_varint().await {
                    0 => InteractionType::Interact,
                    1 => InteractionType::Attack,
                    2 => InteractionType::InteractAt,
                    _ => InteractionType::Interact,
                };

                let target_pos: Option<(f32, f32, f32)> = match interaction_type {
                    InteractionType::InteractAt => Some((
                        self.read_f32().await.unwrap_or_default(),
                        self.read_f32().await.unwrap_or_default(),
                        self.read_f32().await.unwrap_or_default(),
                    )),
                    _ => None,
                };

                let hand: Option<i32> = match interaction_type {
                    InteractionType::Interact | InteractionType::InteractAt => {
                        Some(self.read_varint().await)
                    }
                    _ => None,
                };

                Interact {
                    entity_id,
                    interaction_type,
                    target_pos,
                    hand,
                    sneaking: self.read_u8().await.unwrap_or_default() != 0,
                }
            }
            (ConnectionState::Play, 0x12) => KeepAlive {
                keep_alive_id: self.read_i64().await.unwrap_or_default(),
            },
            (ConnectionState::Play, 0x14) => SetPlayerPosition {
                x: self.read_f64().await.unwrap_or_default(),
                y: self.read_f64().await.unwrap_or_default(),
                z: self.read_f64().await.unwrap_or_default(),
                on_ground: self.read_u8().await.unwrap_or_default() != 0,
            },
            (ConnectionState::Play, 0x15) => SetPlayerPositionAndRotation {
                x: self.read_f64().await.unwrap_or_default(),
                y: self.read_f64().await.unwrap_or_default(),
                z: self.read_f64().await.unwrap_or_default(),
                yaw: self.read_f32().await.unwrap_or_default(),
                pitch: self.read_f32().await.unwrap_or_default(),
                on_ground: self.read_u8().await.unwrap_or_default() != 0,
            },
            (ConnectionState::Play, 0x16) => SetPlayerRotation {
                yaw: self.read_f32().await.unwrap_or_default(),
                pitch: self.read_f32().await.unwrap_or_default(),
                on_ground: self.read_u8().await.unwrap_or_default() != 0,
            },
            (ConnectionState::Play, 0x17) => SetPlayerOnGround {
                on_ground: self.read_u8().await.unwrap_or_default() != 0,
            },
            (_, _) => {
                let mut data: Vec<u8> =
                    vec![0; length as usize - vec![0; 5].write_varint(id).await];
                self.read_exact(&mut data).await.unwrap_or_default();
                Unknown { data }
            }
        }
    }
}

impl<T: AsyncRead + Unpin> ReadPacket for T {}

type Slot = Option<(i32, i8, Value)>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum OutgoingPacket {
    /// Packet ID: 0x00
    StatusResponse { json_response: String },
    /// Packet ID: 0x00
    DisconnectLogin { reason: String },
    /// Packet ID: 0x00
    BundleDelimiter,
    /// Packet ID: 0x01
    PingResponse { payload: i64 },
    /// Packet ID: 0x01
    SpawnEntity {
        entity_id: i32,
        entity_uuid: Uuid,
        entity_type: i32,
        x: f64,
        y: f64,
        z: f64,
        pitch: f32,
        yaw: f32,
        head_yaw: f32,
        data: i32,
        velocity_x: u16,
        velocity_y: u16,
        velocity_z: u16,
    },
    /// Packet ID: 0x02
    LoginSuccess { uuid: Uuid, username: String },
    /// Packet ID: 0x03
    SpawnPlayer {
        entity_id: i32,
        player_uuid: Uuid,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
    },
    /// Packet ID: 0x12
    SetContainerContent {
        window_id: u8,
        state_id: i32,
        slot_data: Vec<Slot>,
        carried_item: Slot,
    },
    /// Packet ID: 0x1B
    DisguisedChatMessage {
        message: String,
        chat_type: i32,
        chat_type_name: String,
        target_name: Option<String>,
    },
    /// Packet ID: 0x23
    KeepAlive { keep_alive_id: i64 },
    /// Packet ID: 0x24
    ChunkDataAndUpdateLight {
        chunk_x: i32,
        chunk_z: i32,
        heightmaps: Value,
        data: Chunk,
        block_entities: Vec<(u8, u16, i32, Value)>,
        sky_light_mask: Vec<i64>,
        block_light_mask: Vec<i64>,
        empty_sky_light_mask: Vec<i64>,
        empty_block_light_mask: Vec<i64>,
        sky_light_arrays: Vec<[u8; 1024]>,
        block_light_arrays: Vec<[u8; 1024]>,
    },
    /// Packet ID: 0x28
    LoginPlay {
        entity_id: i32,
        is_hardcore: bool,
        gamemode: Gamemode,
        previous_gamemode: Option<Gamemode>,
        dimension_names: Vec<String>,
        registry_codec: Vec<u8>,
        dimension_type: String,
        dimension_name: String,
        hashed_seed: i64,
        max_players: i32,
        view_distance: i32,
        simulation_distance: i32,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        is_debug: bool,
        is_flat: bool,
        death_location: Option<(String, Position)>,
        portal_cooldown: i32,
    },
    /// Packet ID: 0x2B
    UpdateEntityPosition {
        entity_id: i32,
        dx: i16,
        dy: i16,
        dz: i16,
        on_ground: bool,
    },
    /// Packet ID: 0x2C
    UpdateEntityPositionAndRotation {
        entity_id: i32,
        dx: i16,
        dy: i16,
        dz: i16,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    /// Packet ID: 0x2D
    UpdateEntityRotation {
        entity_id: i32,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    /// Packet ID: 0x30
    OpenScreen {
        window_id: i32,
        window_type: i32,
        window_title: String,
    },
    /// Packet ID: 0x3A
    PlayerInfoUpdate {
        actions: u8,
        players: Vec<(Uuid, Vec<PlayerActions>)>,
    },
    /// Packet ID: 0x3C
    SynchronizePlayerPosition {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: i8,
        teleport_id: i32,
    },
    /// Packet ID: 0x3E
    RemoveEntities { entity_ids: Vec<i32> },
    /// Packet ID: 0x42
    SetHeadRotation { entity_id: i32, head_yaw: f32 },
    /// Packet ID: 0x4E
    SetCenterChunk { chunk_x: i32, chunk_z: i32 },
    /// Packet ID: 0x50
    SetDefaultSpawnPosition { location: Position, angle: f32 },
    /// Packet ID: 0x52
    SetEntityMetadata {
        entity_id: i32,
        metadata: EntityMetadata,
    },
    /// Packet ID: 0x55
    SetEquipment {
        entity_id: i32,
        equipment: Vec<(u8, Slot)>,
    },
    /// Packet ID: 0x65
    SetTabListHeaderAndFooter { header: String, footer: String },
    /// Packet ID: 0x6A
    UpdateAttributes {
        entity_id: i32,
        properties: Vec<(String, f64, Vec<(Uuid, f64, u8)>)>,
    },
    /// Packet ID: 0x6C
    EntityEffect {
        entity_id: i32,
        effect_id: i32,
        amplifier: u8,
        duration: i32,
        flags: u8,
        factor_codec: Option<Value>,
    },
}

pub trait WritePacket: AsyncWrite + Unpin + Sized {
    async fn write_packet(&mut self, packet: OutgoingPacket) {
        // TODO: double check the code (can be improved)

        // let start: Instant = Instant::now();

        use OutgoingPacket::*;
        let (id, data) = match packet {
            StatusResponse { json_response } => {
                (0x00, types::String(json_response).to_bytes().await)
            }
            DisconnectLogin { reason } => (0x00, types::String(reason).to_bytes().await),
            BundleDelimiter => (0x00, vec![]),
            PingResponse { payload } => (0x01, payload.to_be_bytes().to_vec()),
            SpawnEntity {
                entity_id,
                entity_uuid,
                entity_type,
                x,
                y,
                z,
                pitch,
                yaw,
                head_yaw,
                data,
                velocity_x,
                velocity_y,
                velocity_z,
            } => (0x01, {
                let mut d: Vec<u8> =
                    Vec::with_capacity(5 + 16 + 5 + 8 + 8 + 8 + 1 + 1 + 1 + 5 + 2 + 2 + 2);
                d.write_varint(entity_id).await;
                d.extend_from_slice(&entity_uuid.to_bytes());
                d.write_varint(entity_type).await;
                d.extend_from_slice(&x.to_be_bytes());
                d.extend_from_slice(&y.to_be_bytes());
                d.extend_from_slice(&z.to_be_bytes());
                d.push(Angle::from_deg(pitch).to_angle());
                d.push(Angle::from_deg(yaw).to_angle());
                d.push(Angle::from_deg(head_yaw).to_angle());
                d.write_varint(data).await;
                d.extend_from_slice(&velocity_x.to_be_bytes());
                d.extend_from_slice(&velocity_y.to_be_bytes());
                d.extend_from_slice(&velocity_z.to_be_bytes());
                d
            }),
            LoginSuccess { uuid, username } => (0x02, {
                let mut d: Vec<u8> = Vec::with_capacity(16 + (16 + 1) + 1);
                d.extend_from_slice(&uuid.to_bytes());
                d.write_string(&username).await;
                d.push(0);
                d
            }),
            SpawnPlayer {
                entity_id,
                player_uuid,
                x,
                y,
                z,
                yaw,
                pitch,
            } => (0x03, {
                let mut d: Vec<u8> = Vec::with_capacity(5 + 16 + 8 + 8 + 8 + 1 + 1);
                d.write_varint(entity_id).await;
                d.extend_from_slice(&player_uuid.to_bytes());
                d.extend_from_slice(&x.to_be_bytes());
                d.extend_from_slice(&y.to_be_bytes());
                d.extend_from_slice(&z.to_be_bytes());
                d.push(Angle::from_deg(yaw).to_angle());
                d.push(Angle::from_deg(pitch).to_angle());
                d
            }),
            SetContainerContent {
                window_id,
                state_id,
                mut slot_data,
                carried_item,
            } => (0x12, {
                let mut d: Vec<u8> = Vec::with_capacity(1 + 5 + 1 + slot_data.len() + 1);
                d.push(window_id);
                d.write_varint(state_id).await;
                d.write_varint(slot_data.len() as i32).await;
                slot_data.push(carried_item);
                for slot in slot_data {
                    d.push(slot.is_some() as u8);
                    if let Some((id, count, nbt)) = slot {
                        let nbt: Vec<u8> = fastnbt::to_bytes(&nbt).unwrap_or_default();
                        d.reserve(5 + 1 + nbt.len());
                        d.write_varint(id).await;
                        d.push(count as u8);
                        d.extend_from_slice(&nbt);
                    }
                }
                d
            }),
            DisguisedChatMessage {
                message,
                chat_type,
                chat_type_name,
                target_name,
            } => (0x1B, {
                let mut d: Vec<u8> = Vec::with_capacity(
                    (message.len() + 5)
                        + 5
                        + (chat_type_name.len() + 5)
                        + 1
                        + (target_name
                            .as_ref()
                            .map(|tn| tn.len() + 5)
                            .unwrap_or_default()),
                );
                d.write_string(&message).await;
                d.write_varint(chat_type).await;
                d.write_string(&chat_type_name).await;
                d.push(target_name.is_some() as u8);
                if let Some(tn) = target_name {
                    d.write_string(&tn).await;
                }
                d
            }),
            KeepAlive { keep_alive_id } => (0x23, keep_alive_id.to_be_bytes().to_vec()),
            ChunkDataAndUpdateLight {
                chunk_x,
                chunk_z,
                heightmaps,
                data,
                block_entities,
                sky_light_mask,
                block_light_mask,
                empty_sky_light_mask,
                empty_block_light_mask,
                sky_light_arrays,
                block_light_arrays,
            } => (0x24, {
                let heightmaps: Vec<u8> = fastnbt::to_bytes(&heightmaps).unwrap_or_default();
                let data: Vec<u8> = data.to_bytes().await;
                let block_entities: Vec<u8> = {
                    let mut d: Vec<Vec<u8>> = Vec::with_capacity(block_entities.len());
                    for (xz, y, t, data) in block_entities {
                        let data: Vec<u8> = fastnbt::to_bytes(&data).unwrap_or_default();
                        let mut d_: Vec<u8> = Vec::with_capacity(1 + 2 + 5 + data.len());
                        d_.push(xz);
                        d_.extend_from_slice(&y.to_be_bytes());
                        d_.write_varint(t).await;
                        d_.extend_from_slice(&data);
                        d.push(d_);
                    }
                    d.concat()
                };

                let mut d: Vec<u8> = Vec::with_capacity(
                    4 + 4
                        + heightmaps.len()
                        + (5 + data.len())
                        + (5 + block_entities.len())
                        + (5 + sky_light_mask.len() * 8)
                        + (5 + block_light_mask.len() * 8)
                        + (5 + empty_sky_light_mask.len() * 8)
                        + (5 + empty_block_light_mask.len() * 8)
                        + (5 + sky_light_arrays.len() * 1024)
                        + (5 + 1024),
                );
                d.extend_from_slice(&chunk_x.to_be_bytes());
                d.extend_from_slice(&chunk_z.to_be_bytes());
                d.extend_from_slice(&heightmaps);
                d.write_varint(data.len() as i32).await;
                d.extend_from_slice(&data);
                d.write_varint(block_entities.len() as i32).await;
                d.extend_from_slice(&block_entities);
                d.write_varint(sky_light_mask.len() as i32).await;
                for i in sky_light_mask {
                    d.extend_from_slice(&i.to_be_bytes());
                }
                d.write_varint(block_light_mask.len() as i32).await;
                for i in block_light_mask {
                    d.extend_from_slice(&i.to_be_bytes());
                }
                d.write_varint(empty_sky_light_mask.len() as i32).await;
                for i in empty_sky_light_mask {
                    d.extend_from_slice(&i.to_be_bytes());
                }
                d.write_varint(empty_block_light_mask.len() as i32).await;
                for i in empty_block_light_mask {
                    d.extend_from_slice(&i.to_be_bytes());
                }
                d.write_varint(sky_light_arrays.len() as i32).await;
                for i in sky_light_arrays {
                    d.extend_from_slice(&i);
                }
                d.write_varint(block_light_arrays.len() as i32).await;
                for i in block_light_arrays {
                    d.extend_from_slice(&i);
                }
                d
            }),
            LoginPlay {
                entity_id,
                is_hardcore,
                gamemode,
                previous_gamemode,
                dimension_names,
                registry_codec,
                dimension_type,
                dimension_name,
                hashed_seed,
                max_players,
                view_distance,
                simulation_distance,
                reduced_debug_info,
                enable_respawn_screen,
                is_debug,
                is_flat,
                death_location,
                portal_cooldown,
            } => (0x28, {
                let dimension_names: Vec<u8> = {
                    let mut d: Vec<u8> = Vec::with_capacity(5);
                    d.write_varint(dimension_names.len() as i32).await;
                    for dn in dimension_names {
                        d.reserve(5 + dn.len());
                        d.write_string(&dn).await;
                    }
                    d
                };
                let mut d: Vec<u8> = Vec::with_capacity(
                    5 + 1
                        + 1
                        + 1
                        + (5 + dimension_names.len())
                        + registry_codec.len()
                        + (5 + dimension_type.len())
                        + (5 + dimension_name.len())
                        + 8
                        + 5
                        + 5
                        + 5
                        + 1
                        + 1
                        + 1
                        + 1
                        + 1
                        + (death_location
                            .as_ref()
                            .map(|(ddn, _)| (5 + ddn.len()) + 8)
                            .unwrap_or_default())
                        + 5,
                );
                d.extend_from_slice(&entity_id.to_be_bytes());
                d.push(is_hardcore as u8);
                d.push(gamemode as u8);
                d.push(previous_gamemode.map_or(255, |gm: Gamemode| gm as u8));
                d.extend_from_slice(&dimension_names);
                d.extend_from_slice(&registry_codec);
                d.write_string(&dimension_type).await;
                d.write_string(&dimension_name).await;
                d.extend_from_slice(&hashed_seed.to_be_bytes());
                d.write_varint(max_players).await;
                d.write_varint(view_distance).await;
                d.write_varint(simulation_distance).await;
                d.extend_from_slice(&[
                    reduced_debug_info as u8,
                    enable_respawn_screen as u8,
                    is_debug as u8,
                    is_flat as u8,
                    death_location.is_some() as u8,
                ]);
                if let Some((ddn, dl)) = death_location {
                    d.write_string(&ddn).await;
                    d.extend_from_slice(&dl.to_bytes());
                }
                d.write_varint(portal_cooldown).await;
                d
            }),
            UpdateEntityPosition {
                entity_id,
                dx,
                dy,
                dz,
                on_ground,
            } => (0x2B, {
                let mut d: Vec<u8> = Vec::with_capacity(5 + 2 + 2 + 2 + 1);
                d.write_varint(entity_id).await;
                d.extend_from_slice(&dx.to_be_bytes());
                d.extend_from_slice(&dy.to_be_bytes());
                d.extend_from_slice(&dz.to_be_bytes());
                d.push(on_ground as u8);
                d
            }),
            UpdateEntityPositionAndRotation {
                entity_id,
                dx,
                dy,
                dz,
                yaw,
                pitch,
                on_ground,
            } => (0x2C, {
                let mut d: Vec<u8> = Vec::with_capacity(5 + 2 + 2 + 2 + 1 + 1 + 1);
                d.write_varint(entity_id).await;
                d.extend_from_slice(&dx.to_be_bytes());
                d.extend_from_slice(&dy.to_be_bytes());
                d.extend_from_slice(&dz.to_be_bytes());
                d.push(Angle::from_deg(yaw).to_angle());
                d.push(Angle::from_deg(pitch).to_angle());
                d.push(on_ground as u8);
                d
            }),
            UpdateEntityRotation {
                entity_id,
                yaw,
                pitch,
                on_ground,
            } => (0x2D, {
                let mut d: Vec<u8> = Vec::with_capacity(5 + 1 + 1 + 1);
                d.write_varint(entity_id).await;
                d.push(Angle::from_deg(yaw).to_angle());
                d.push(Angle::from_deg(pitch).to_angle());
                d.push(on_ground as u8);
                d
            }),
            OpenScreen {
                window_id,
                window_type,
                window_title,
            } => (0x30, {
                let mut d: Vec<u8> = Vec::with_capacity(5 + 5 + 5 + window_title.len());
                d.write_varint(window_id).await;
                d.write_varint(window_type as i32).await;
                d.write_string(&window_title).await;
                d
            }),
            SynchronizePlayerPosition {
                x,
                y,
                z,
                yaw,
                pitch,
                flags,
                teleport_id,
            } => (0x3C, {
                let mut d: Vec<u8> = Vec::with_capacity(8 + 8 + 8 + 4 + 4 + 1 + 5);
                d.extend_from_slice(&x.to_be_bytes());
                d.extend_from_slice(&y.to_be_bytes());
                d.extend_from_slice(&z.to_be_bytes());
                d.extend_from_slice(&yaw.to_be_bytes());
                d.extend_from_slice(&pitch.to_be_bytes());
                d.push(flags as u8);
                d.write_varint(teleport_id).await;
                d
            }),
            RemoveEntities { entity_ids } => (0x3E, {
                let mut d: Vec<u8> = Vec::with_capacity(5 + entity_ids.len() * 5);
                d.write_varint(entity_ids.len() as i32).await;
                for entity_id in entity_ids {
                    d.write_varint(entity_id).await;
                }
                d
            }),
            SetHeadRotation {
                entity_id,
                head_yaw,
            } => (0x42, {
                let mut d: Vec<u8> = Vec::with_capacity(5 + 1);
                d.write_varint(entity_id).await;
                d.push(Angle::from_deg(head_yaw).to_angle());
                d
            }),
            PlayerInfoUpdate { actions, players } => (0x3A, {
                let players: Vec<u8> = {
                    let mut d: Vec<u8> = Vec::with_capacity(5);
                    d.write_varint(players.len() as i32).await;
                    for (uuid, pas) in players {
                        d.reserve(16);
                        d.extend_from_slice(&uuid.to_bytes());
                        for pa in pas {
                            let d_: Vec<u8> = pa.to_bytes().await;
                            d.reserve(d_.len());
                            d.extend_from_slice(&d_);
                        }
                    }
                    d
                };
                let mut d: Vec<u8> = Vec::with_capacity(1 + (5 + players.len()));
                d.push(actions);
                d.extend_from_slice(&players);
                d
            }),
            SetCenterChunk { chunk_x, chunk_z } => (0x4E, {
                let mut d: Vec<u8> = Vec::with_capacity(5 + 5);
                d.write_varint(chunk_x).await;
                d.write_varint(chunk_z).await;
                d
            }),
            SetDefaultSpawnPosition { location, angle } => (0x50, {
                let mut d: Vec<u8> = Vec::with_capacity(8 + 4);
                d.extend_from_slice(&location.to_bytes());
                d.extend_from_slice(&angle.to_be_bytes());
                d
            }),
            SetEntityMetadata {
                entity_id,
                metadata,
            } => (0x52, {
                let mut d: Vec<u8> = Vec::with_capacity(5);
                d.write_varint(entity_id).await;
                for (index, field) in metadata.0 {
                    let field: Vec<u8> = field.to_bytes().await;
                    d.reserve(1 + field.len());
                    d.push(index);
                    d.extend_from_slice(&field);
                }
                d.reserve(1);
                d.push(0xFF);
                d
            }),
            SetEquipment {
                entity_id,
                equipment,
            } => (0x55, {
                let mut d: Vec<u8> = Vec::with_capacity(6);
                d.write_varint(entity_id).await;
                for (slot, item) in equipment {
                    d.push(slot);
                    d.push(item.is_some() as u8);
                    if let Some((id, count, nbt)) = item {
                        let nbt: Vec<u8> = fastnbt::to_bytes(&nbt).unwrap_or_default();
                        d.reserve(5 + 1 + nbt.len());
                        d.write_varint(id).await;
                        d.push(count as u8);
                        d.extend_from_slice(&nbt);
                    }
                }
                d
            }),
            SetTabListHeaderAndFooter { header, footer } => (0x65, {
                let mut d: Vec<u8> = Vec::with_capacity((5 + header.len()) + (5 + footer.len()));
                d.write_string(&header).await;
                d.write_string(&footer).await;
                d
            }),
            UpdateAttributes {
                entity_id,
                properties,
            } => (0x6A, {
                let properties: Vec<u8> = {
                    let mut d: Vec<u8> = Vec::with_capacity(5);
                    d.write_varint(properties.len() as i32).await;
                    for (key, value, modifiers) in properties {
                        d.reserve((5 + key.len()) + 8 + 5 + modifiers.len() * (16 + 8 + 1));
                        d.write_string(&key).await;
                        d.extend_from_slice(&value.to_be_bytes());
                        d.write_varint(modifiers.len() as i32).await;
                        for (uuid, amount, operation) in modifiers {
                            d.extend_from_slice(&uuid.to_bytes());
                            d.extend_from_slice(&amount.to_be_bytes());
                            d.push(operation);
                        }
                    }
                    d
                };
                let mut d: Vec<u8> = Vec::with_capacity(5 + (5 + properties.len()));
                d.write_varint(entity_id).await;
                d.extend_from_slice(&properties);
                d
            }),
            EntityEffect {
                entity_id,
                effect_id,
                amplifier,
                duration,
                flags,
                factor_codec,
            } => (0x6C, {
                let factor_codec: Option<Vec<u8>> =
                    factor_codec.map(|fc: Value| fastnbt::to_bytes(&fc).unwrap_or_default());
                let mut d: Vec<u8> = Vec::with_capacity(
                    5 + 5
                        + 1
                        + 5
                        + 1
                        + (1 + factor_codec.as_ref().map_or(0, |fc: &Vec<u8>| fc.len())),
                );
                d.write_varint(entity_id).await;
                d.write_varint(effect_id).await;
                d.push(amplifier);
                d.write_varint(duration).await;
                d.push(flags);
                d.push(factor_codec.is_some() as u8);
                if let Some(factor_codec) = factor_codec {
                    d.extend_from_slice(&factor_codec);
                }
                d
            }),
        };

        // debug!(
        //     "Packet serialization time: {:?}ns",
        //     start.elapsed().as_nanos()
        // );

        // let start: Instant = Instant::now();

        // NOTE: this is a performance hack and it will break if packet id can be more than 127
        // let id: Vec<u8> = {
        //     let mut d: Vec<u8> = Vec::with_capacity(5);
        //     d.write_varint(id).await;
        //     d
        // };
        self.write_varint((1 + data.len()) as i32).await;
        // self.write_all(&id).await.unwrap_or_default();
        self.write_u8(id as u8).await.unwrap_or_default();
        self.write(&data).await.unwrap_or_default();

        // debug!("Packet write time: {:?}ns", start.elapsed().as_nanos());
    }

    async fn write_packets(&mut self, packets: Vec<OutgoingPacket>) {
        if packets.is_empty() {
            return;
        }

        // let mut times: Vec<Duration> = vec![];
        let mut packets: VecDeque<OutgoingPacket> = VecDeque::from(packets);
        for _ in 0..packets.len().div_ceil(4096) {
            self.write_packet(OutgoingPacket::BundleDelimiter).await;
            for _ in 0..4096.min(packets.len()) {
                // let start: Instant = Instant::now();
                self.write_packet(packets.pop_front().unwrap()).await;
                // times.push(start.elapsed());
            }
            self.write_packet(OutgoingPacket::BundleDelimiter).await;
        }
        // debug!(
        //     "Average packet send time: {:?}ns",
        //     // "Average packet write time: {:?}µs",
        //     times.iter().map(|d| d.as_nanos()).sum::<u128>() as f64 / times.len() as f64
        // );
    }

    async fn write_packets_nonbundling(&mut self, packets: Vec<OutgoingPacket>) {
        for packet in packets {
            self.write_packet(packet).await;
        }
    }
}

impl<T: AsyncWrite + Unpin + Sized> WritePacket for T {}

pub async fn prefix_with_length(packet: &mut Vec<u8>) {
    let len_bytes: usize = packet.write_varint(packet.len() as i32).await;
    packet.rotate_right(len_bytes);
}
