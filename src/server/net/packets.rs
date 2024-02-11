#![allow(dead_code)]

use std::{fmt::Display, string::String};

use fastnbt::Value;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use uuid::Uuid;

use crate::server::{
    state::ConnectionState,
    types::{
        self, Angle, Gamemode, PlayerActions, Position, ReadString, ReadVarInt, VarInt, WriteVarInt,
    },
};

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
        let length: i32 = self.read_var_int().await;
        if length == 0 {
            return IncomingPacket::Unknown { data: vec![] };
        }
        let id: i32 = self.read_var_int().await;

        {
            use IncomingPacket::*;
            match (state, id) {
                (ConnectionState::Handshake, 0x00) => Handshake {
                    protocol_version: self.read_var_int().await,
                    server_address: self.read_string().await,
                    server_port: self.read_u16().await.unwrap_or_default(),
                    next_state: self.read_var_int().await,
                },
                (ConnectionState::Status, 0x00) => StatusRequest,
                (ConnectionState::Ping | ConnectionState::Status, 0x01) => PingRequest {
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
                        vec![0; length as usize - vec![0; 5].write_var_int(id).await];
                    self.read_exact(&mut data).await.unwrap_or_default();
                    Unknown { data }
                }
            }
        }
    }
}
impl<T: AsyncRead + Unpin> ReadPacket for T {}

#[derive(Debug)]
pub enum OutgoingPacket {
    /// Packet ID: 0x00
    StatusResponse { json_response: String },
    /// Packet ID: 0x00
    DisconnectLogin { reason: String },
    /// Packet ID: 0x00
    BundleDelimiter,
    /// Packet ID: 0x01
    PingResponse { payload: i64 },
    /// Packet ID: 0x02
    LoginSuccess { uuid: u128, username: String },
    /// Packet ID: 0x03
    SpawnPlayer {
        entity_id: i32,
        player_uuid: u128,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
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
        data: Vec<u8>,
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
    /// Packet ID: 0x3A
    PlayerInfoUpdate {
        actions: u8,
        players: Vec<(u128, Vec<PlayerActions>)>,
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
    /// Packet ID: 0x42
    SetHeadRotation { entity_id: i32, head_yaw: f32 },
    /// Packet ID: 0x4E
    SetCenterChunk { chunk_x: i32, chunk_z: i32 },
    /// Packet ID: 0x50
    SetDefaultSpawnPosition { location: Position, angle: f32 },
    /// Packet ID: 0x65
    SetTabListHeaderAndFooter { header: String, footer: String },
    /// Packet ID: 0x6A
    UpdateAttributes {
        entity_id: i32,
        properties: Vec<(types::String, f64, Vec<(Uuid, f64, u8)>)>,
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
        use OutgoingPacket::*;
        let (id, data) = match packet {
            StatusResponse { json_response } => {
                (0x00, types::String(json_response).to_bytes().await)
            }
            DisconnectLogin { reason } => (0x00, types::String(reason).to_bytes().await),
            BundleDelimiter => (0x00, vec![]),
            PingResponse { payload } => (0x01, payload.to_be_bytes().into()),
            LoginSuccess { uuid, username } => (
                0x02,
                [
                    &uuid.to_be_bytes()[..],
                    &VarInt(username.len() as i32).to_bytes().await,
                    username.as_bytes(),
                    &[0], // Some unknown properties
                ]
                .concat(),
            ),
            SpawnPlayer {
                entity_id,
                player_uuid,
                x,
                y,
                z,
                yaw,
                pitch,
            } => (
                0x03,
                [
                    &VarInt(entity_id).to_bytes().await[..],
                    &player_uuid.to_be_bytes(),
                    &x.to_be_bytes(),
                    &y.to_be_bytes(),
                    &z.to_be_bytes(),
                    &[
                        Angle::from_deg(yaw).to_angle(),
                        Angle::from_deg(pitch).to_angle(),
                    ],
                ]
                .concat(),
            ),
            DisguisedChatMessage {
                message,
                chat_type,
                chat_type_name,
                target_name,
            } => (
                0x1B,
                [
                    message.as_bytes(),
                    &VarInt(chat_type).to_bytes().await,
                    chat_type_name.as_bytes(),
                    &[target_name.is_some() as u8],
                    &target_name
                        .map(|tn| tn.as_bytes().to_vec())
                        .unwrap_or_default(),
                ]
                .concat(),
            ),
            KeepAlive { keep_alive_id } => (0x23, keep_alive_id.to_be_bytes().into()),
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
            } => (
                0x24,
                [
                    &chunk_x.to_be_bytes()[..],
                    &chunk_z.to_be_bytes(),
                    &fastnbt::to_bytes(&heightmaps).unwrap_or_default(),
                    &VarInt(data.len() as i32).to_bytes().await,
                    &data,
                    &VarInt(block_entities.len() as i32).to_bytes().await,
                    {
                        let mut buf: Vec<Vec<u8>> = Vec::new();
                        for (xz, y, t, d) in block_entities {
                            buf.push(
                                [
                                    &[xz][..],
                                    &y.to_be_bytes(),
                                    &VarInt(t).to_bytes().await,
                                    &fastnbt::to_bytes(&d).unwrap_or_default(),
                                ]
                                .concat(),
                            );
                        }
                        &buf.concat()
                    },
                    &VarInt(sky_light_mask.len() as i32).to_bytes().await,
                    &sky_light_mask
                        .iter()
                        .map(|l| l.to_be_bytes())
                        .collect::<Vec<[u8; 8]>>()
                        .concat(),
                    &VarInt(block_light_mask.len() as i32).to_bytes().await,
                    &block_light_mask
                        .iter()
                        .map(|l| l.to_be_bytes())
                        .collect::<Vec<[u8; 8]>>()
                        .concat(),
                    &VarInt(empty_sky_light_mask.len() as i32).to_bytes().await,
                    &empty_sky_light_mask
                        .iter()
                        .map(|l| l.to_be_bytes())
                        .collect::<Vec<[u8; 8]>>()
                        .concat(),
                    &VarInt(empty_block_light_mask.len() as i32).to_bytes().await,
                    &empty_block_light_mask
                        .iter()
                        .map(|l| l.to_be_bytes())
                        .collect::<Vec<[u8; 8]>>()
                        .concat(),
                    &VarInt(sky_light_arrays.len() as i32).to_bytes().await,
                    &sky_light_arrays.concat(),
                    &VarInt(block_light_arrays.len() as i32).to_bytes().await,
                    &block_light_arrays.concat(),
                ]
                .concat(),
            ),
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
            } => (
                0x28,
                [
                    &entity_id.to_be_bytes()[..],
                    &[
                        is_hardcore as u8,
                        gamemode as u8,
                        previous_gamemode
                            .map(|gm: Gamemode| gm as u8)
                            .unwrap_or(255),
                    ],
                    &VarInt(dimension_names.len() as i32).to_bytes().await,
                    {
                        let mut buf: Vec<Vec<u8>> = vec![];
                        for dimension_name in dimension_names {
                            buf.push(types::String(dimension_name).to_bytes().await)
                        }
                        &buf.concat()
                    },
                    &registry_codec,
                    &types::String(dimension_type).to_bytes().await,
                    &types::String(dimension_name).to_bytes().await,
                    &hashed_seed.to_be_bytes(),
                    &VarInt(max_players).to_bytes().await,
                    &VarInt(view_distance).to_bytes().await,
                    &VarInt(simulation_distance).to_bytes().await,
                    &[
                        reduced_debug_info as u8,
                        enable_respawn_screen as u8,
                        is_debug as u8,
                        is_flat as u8,
                        death_location.is_some() as u8,
                    ],
                    {
                        let buf: Vec<u8> = match death_location {
                            Some(death_location) => [
                                &types::String(death_location.0).to_bytes().await[..],
                                &death_location.1.to_bytes(),
                            ]
                            .concat(),
                            None => vec![],
                        };
                        &buf.clone()
                    },
                    &VarInt(portal_cooldown).to_bytes().await,
                ]
                .concat(),
            ),
            UpdateEntityPosition {
                entity_id,
                dx,
                dy,
                dz,
                on_ground,
            } => (
                0x2B,
                [
                    &VarInt(entity_id).to_bytes().await[..],
                    &dx.to_be_bytes(),
                    &dy.to_be_bytes(),
                    &dz.to_be_bytes(),
                    &[on_ground as u8],
                ]
                .concat(),
            ),
            UpdateEntityPositionAndRotation {
                entity_id,
                dx,
                dy,
                dz,
                yaw,
                pitch,
                on_ground,
            } => (
                0x2C,
                [
                    &VarInt(entity_id).to_bytes().await[..],
                    &dx.to_be_bytes(),
                    &dy.to_be_bytes(),
                    &dz.to_be_bytes(),
                    &[
                        Angle::from_deg(yaw).to_angle(),
                        // Angle(pitch).to_angle(),
                        Angle::from_deg(pitch).to_angle(),
                        on_ground as u8,
                    ],
                ]
                .concat(),
            ),
            UpdateEntityRotation {
                entity_id,
                yaw,
                pitch,
                on_ground,
            } => (
                0x2D,
                [
                    &VarInt(entity_id).to_bytes().await[..],
                    &[
                        Angle::from_deg(yaw).to_angle(),
                        Angle::from_deg(pitch).to_angle(),
                        on_ground as u8,
                    ],
                ]
                .concat(),
            ),
            SynchronizePlayerPosition {
                x,
                y,
                z,
                yaw,
                pitch,
                flags,
                teleport_id,
            } => (
                0x3C,
                [
                    &x.to_be_bytes()[..],
                    &y.to_be_bytes(),
                    &z.to_be_bytes(),
                    &yaw.to_be_bytes(),
                    &pitch.to_be_bytes(),
                    &[flags as u8],
                    &VarInt(teleport_id).to_bytes().await,
                ]
                .concat(),
            ),
            SetHeadRotation {
                entity_id,
                head_yaw,
            } => (
                0x42,
                [
                    &VarInt(entity_id).to_bytes().await[..],
                    &[Angle::from_deg(head_yaw).to_angle()],
                ]
                .concat(),
            ),
            PlayerInfoUpdate { actions, players } => (
                0x3A,
                [
                    &[actions][..],
                    &VarInt(players.len() as i32).to_bytes().await,
                    {
                        let mut buf: Vec<Vec<u8>> = vec![];
                        for player in players {
                            buf.push(
                                [&player.0.to_be_bytes()[..], {
                                    let mut buf: Vec<Vec<u8>> = vec![];
                                    for player_actions in player.1 {
                                        buf.push(player_actions.to_bytes().await);
                                    }
                                    &buf.concat()
                                }]
                                .concat(),
                            );
                        }
                        &buf.concat()
                    },
                ]
                .concat(),
            ),
            SetCenterChunk { chunk_x, chunk_z } => (
                0x4E,
                [
                    VarInt(chunk_x).to_bytes().await,
                    VarInt(chunk_z).to_bytes().await,
                ]
                .concat(),
            ),
            SetDefaultSpawnPosition { location, angle } => (
                0x50,
                [&location.to_bytes()[..], &angle.to_be_bytes()[..]].concat(),
            ),
            SetTabListHeaderAndFooter { header, footer } => (
                0x65,
                [
                    &types::String(header).to_bytes().await[..],
                    &types::String(footer).to_bytes().await,
                ]
                .concat(),
            ),
            UpdateAttributes {
                entity_id,
                properties,
            } => (
                0x6A,
                [
                    &VarInt(entity_id).to_bytes().await[..],
                    &VarInt(properties.len() as i32).to_bytes().await,
                    {
                        let mut buf: Vec<Vec<u8>> = vec![];
                        for (key, value, modifiers) in properties {
                            buf.push(
                                [
                                    &key.to_bytes().await[..],
                                    &value.to_be_bytes(),
                                    &VarInt(modifiers.len() as i32).to_bytes().await,
                                    {
                                        let mut buf: Vec<Vec<u8>> = vec![];
                                        for (uuid, amount, operation) in modifiers {
                                            buf.push(
                                                [
                                                    &uuid.as_bytes()[..],
                                                    &amount.to_be_bytes(),
                                                    &[operation],
                                                ]
                                                .concat(),
                                            );
                                        }
                                        &buf.concat()
                                    },
                                ]
                                .concat(),
                            );
                        }
                        &buf.concat()
                    },
                ]
                .concat(),
            ),
            EntityEffect {
                entity_id,
                effect_id,
                amplifier,
                duration,
                flags,
                factor_codec,
            } => (
                0x6C,
                [
                    &VarInt(entity_id).to_bytes().await[..],
                    &VarInt(effect_id).to_bytes().await,
                    &[amplifier],
                    &VarInt(duration).to_bytes().await,
                    &[flags, factor_codec.is_some() as u8],
                    &factor_codec
                        .map(|fc| fastnbt::to_bytes(&fc).unwrap_or_default())
                        .unwrap_or_default(),
                ]
                .concat(),
            ),
        };
        let id: Vec<u8> = VarInt(id).to_bytes().await;
        self.write_var_int((id.len() + data.len()) as i32).await;
        self.write_all(&id).await.unwrap_or_default();
        self.write_all(&data).await.unwrap_or_default();
    }

    async fn write_packets(&mut self, mut packets: Vec<OutgoingPacket>) {
        if packets.len() > 1 {
            self.write_packet(OutgoingPacket::BundleDelimiter).await;
            packets.push(OutgoingPacket::BundleDelimiter);
        }
        for packet in packets {
            self.write_packet(packet).await;
        }
    }
}
impl<T: AsyncWrite + Unpin + Sized> WritePacket for T {}

pub async fn prefix_with_length(packet: &mut Vec<u8>) {
    let len_bytes: usize = packet.write_var_int(packet.len() as i32).await;
    packet.rotate_right(len_bytes);
}
