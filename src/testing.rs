use log::debug;

use crate::server::net::packets::OutgoingPacket;

pub async fn test() {
    debug!("{}", std::mem::size_of::<OutgoingPacket>());

    // let chunk_section: ChunkSection = ChunkSection {
    //     blocks: vec![Block::Stone],
    // };

    // debug!("{:?}", chunk_section.to_bytes());
}

// #[allow(dead_code)]
// async fn handle_connection(
//     mut connection: TcpStream,
//     config: Arc<Mutex<Config>>,
//     chunk_data: Arc<Mutex<ChunkData>>,
//     entity_ids: Arc<Mutex<Vec<i32>>>,
//     player_data: Arc<Mutex<PlayerData>>,
//     events: Arc<Mutex<Vec<Event>>>,
// ) {
//     // TODO: Move everything besides play out of the loop
//     //? Split the stream in halves?
//     //? Read packets directly from the connection (and spawn a task to send keep alive packets for now)?

//     // let (read_half, write_half) = connection.split();
//     // let (mut read_half, mut write_half) = (BufReader::new(read_half), BufWriter::new(write_half));

//     let timeout: u64 = 5;
//     let entity_id: i32 = gen_entity_id(entity_ids.clone()).await;

//     let mut tick_interval: Interval = interval(Duration::from_nanos(1));
//     let mut playstate_tick: u128 = 0;
//     let mut state: ConnectionState = ConnectionState::Handshake;
//     let mut last_timeout: Instant = Instant::now();
//     let mut player: Player = Player {
//         id: entity_id,
//         name: "".to_string(),
//         x: 8.0,
//         y: 320.0,
//         z: 8.0,
//         yaw: 0.0,
//         pitch: 0.0,
//         on_ground: true,
//     };
//     let mut event_index: usize = events.lock().await.len();
//     let mut center_chunk: (i32, i32) = (0, 0);

//     let mut tick_times: Vec<Duration> = vec![];

//     // Infinite connection
//     'conn: loop {
//         tick_interval.tick().await;

//         let start: Instant = Instant::now();

//         // Send keep alive
//         if state == ConnectionState::Play {
//             // debug!("Playstate tick #{playstate_tick} of player {}", player.name);
//             playstate_tick += 1;
//             if playstate_tick % 1_000_000 == 0 {
//                 // debug!("Sending keep alive");
//                 connection
//                     .write_packet(OutgoingPacket::KeepAlive {
//                         keep_alive_id: random(),
//                     })
//                     .await;
//             }
//         }

//         // Check timeout
//         if last_timeout.elapsed().as_secs() > timeout {
//             debug!("Timed out");
//             break 'conn;
//         }

//         let mut moved: bool = false;

//         // Get all incoming data
//         let mut incoming: Cursor<Vec<u8>> = Cursor::new(vec![]);
//         let mut buf: [u8; 1] = [0];
//         match connection.try_read(&mut buf) {
//             Ok(0) => {
//                 debug!("Connection closed");
//                 break 'conn;
//             }
//             Ok(_) => {
//                 incoming.get_mut().extend_from_slice(&buf);
//                 loop {
//                     let mut buf: [u8; 4096] = [0; 4096];
//                     let n: usize = connection.try_read(&mut buf).unwrap_or_default();
//                     if n == 0 {
//                         break;
//                     }
//                     incoming.get_mut().extend_from_slice(&buf[..n]);
//                 }

//                 // Handle all incoming packets
//                 loop {
//                     // Read the packet
//                     let packet: IncomingPacket = incoming.read_packet(&state).await;

//                     // debug!(
//                     //     "NEW PACKET | State: {state:?}{} | From: {}{} | Type: {packet}",
//                     //     " ".repeat(9 - format!("{state:?}").len()),
//                     //     player.name,
//                     //     " ".repeat(16 - player.name.len()),
//                     // );

//                     use IncomingPacket::*;
//                     match packet {
//                         Unknown { data } if data.is_empty() => break,
//                         Unknown { .. } => continue,
//                         Handshake {
//                             protocol_version: _,
//                             server_address: _,
//                             server_port: _,
//                             next_state,
//                         } => {
//                             last_timeout = Instant::now();
//                             state = match next_state {
//                                 0x01 => ConnectionState::Status,
//                                 0x02 => ConnectionState::Login,
//                                 _ => {
//                                     debug!("Invalid state");
//                                     break 'conn;
//                                 }
//                             }
//                         }
//                         StatusRequest {} => {
//                             connection
//                                 .write_packet(OutgoingPacket::StatusResponse {
//                                     json_response: serde_json::to_string(
//                                         &config.lock().await.status,
//                                     )
//                                     .unwrap_or_default(),
//                                 })
//                                 .await;
//                             last_timeout = Instant::now();
//                         }
//                         PingRequest { payload } => {
//                             connection
//                                 .write_packet(OutgoingPacket::PingResponse { payload })
//                                 .await;
//                             debug!("Ping request");
//                             break 'conn;
//                         }
//                         LoginStart { name, .. } => {
//                             let mut player_data: MutexGuard<'_, HashMap<i32, Player>> =
//                                 player_data.lock().await;

//                             config.lock().await.status.players.online += 1;
//                             events.lock().await.push(Event::PlayerJoin(entity_id));

//                             player.name = name;
//                             player_data.insert(entity_id, player.clone());
//                             state = ConnectionState::Play;

//                             connection
//                                 .write_packet(OutgoingPacket::LoginSuccess {
//                                     uuid: Uuid::offline(player.name.to_string()),
//                                     username: player.name.to_string(),
//                                 })
//                                 .await;

//                             use OutgoingPacket::*;
//                             use PlayerActions::*;
//                             let mut packets: Vec<OutgoingPacket> = vec![];

//                             packets.push(LoginPlay {
//                                 entity_id: player.id,
//                                 is_hardcore: false,
//                                 gamemode: Gamemode::Creative,
//                                 previous_gamemode: None,
//                                 dimension_names: vec!["minecraft:overworld".to_string()],
//                                 registry_codec: REGISTRY_CODEC.clone(),
//                                 dimension_type: "minecraft:overworld".to_string(),
//                                 dimension_name: "minecraft:overworld".to_string(),
//                                 hashed_seed: *HASHED_SEED,
//                                 max_players: 20,
//                                 view_distance: 16,
//                                 simulation_distance: 12,
//                                 reduced_debug_info: false,
//                                 enable_respawn_screen: true,
//                                 is_debug: false,
//                                 is_flat: true,
//                                 death_location: None,
//                                 portal_cooldown: 0,
//                             });
//                             packets.push(PlayerInfoUpdate {
//                                 actions: 0x01 | 0x08,
//                                 players: player_data
//                                     .values()
//                                     .map(|p| {
//                                         (
//                                             Uuid::offline(p.name.to_string()),
//                                             vec![
//                                                 AddPlayer {
//                                                     name: p.name.to_string(),
//                                                     properties: vec![],
//                                                 },
//                                                 UpdateListed { listed: true },
//                                             ],
//                                         )
//                                     })
//                                     .collect(),
//                             });
//                             packets.push(SetCenterChunk {
//                                 chunk_x: 0,
//                                 chunk_z: 0,
//                             });
//                             {
//                                 let chunk_data = chunk_data.lock().await;
//                                 for x in -16..=16 {
//                                     for z in -16..=16 {
//                                         center_chunk = (0, 0);
//                                         packets.push(ChunkDataAndUpdateLight {
//                                             chunk_x: x,
//                                             chunk_z: z,
//                                             heightmaps: nbt!({}),
//                                             data: chunk_data.get(&(x, z)).cloned().unwrap(),
//                                             block_entities: vec![],
//                                             sky_light_mask: vec![],
//                                             block_light_mask: vec![],
//                                             empty_sky_light_mask: vec![],
//                                             empty_block_light_mask: vec![],
//                                             sky_light_arrays: vec![],
//                                             block_light_arrays: vec![],
//                                         });
//                                     }
//                                 }
//                             }
//                             packets.push(SetDefaultSpawnPosition {
//                                 location: Position { x: 8, y: 320, z: 8 },
//                                 angle: 0.0,
//                             });
//                             packets.push(SynchronizePlayerPosition {
//                                 x: player.x,
//                                 y: player.y,
//                                 z: player.z,
//                                 yaw: player.yaw,
//                                 pitch: player.pitch,
//                                 flags: 0,
//                                 teleport_id: 0,
//                             });
//                             packets.push(EntityEffect {
//                                 entity_id: player.id,
//                                 effect_id: 1,
//                                 amplifier: 4,
//                                 duration: -1,
//                                 flags: 0x02 | 0x04,
//                                 factor_codec: None,
//                             });
//                             packets.push(EntityEffect {
//                                 entity_id: player.id,
//                                 effect_id: 16,
//                                 amplifier: 0,
//                                 duration: -1,
//                                 flags: 0x02 | 0x04,
//                                 factor_codec: None,
//                             });
//                             packets.push(UpdateAttributes {
//                                 entity_id: player.id,
//                                 properties: vec![(
//                                     "minecraft:generic.movement_speed".to_string(),
//                                     0.1,
//                                     vec![(
//                                         Uuid(193644973882603813127438245391424226869),
//                                         1.0,
//                                         0x01,
//                                     )],
//                                 )],
//                             });
//                             packets.push(SetTabListHeaderAndFooter {
//                                 header: "{\"text\": \"\nUSSR 1.20.1\n\"}".to_string(),
//                                 footer: "{\"text\": \"\n\"}".to_string(),
//                             });
//                             for p in player_data.values() {
//                                 if p.id == entity_id {
//                                     continue;
//                                 }
//                                 packets.push(SpawnPlayer {
//                                     entity_id: p.id,
//                                     player_uuid: Uuid::offline(p.name.to_string()),
//                                     x: p.x,
//                                     y: p.y,
//                                     z: p.z,
//                                     yaw: p.yaw,
//                                     pitch: p.pitch,
//                                 });
//                             }

//                             connection.write_packets(packets).await;

//                             last_timeout = Instant::now();
//                         }
//                         Interact { entity_id, .. } => {
//                             let (x, _, z) = player.get_block();
//                             let y: f64 = chunk_data
//                                 .lock()
//                                 .await
//                                 .get(&player.get_chunk())
//                                 .unwrap()
//                                 .max_height_at(x, z)
//                                 .unwrap_or(319) as f64;
//                             events.lock().await.push(Event::PlayerMove(
//                                 entity_id,
//                                 0,
//                                 ((y * 32.0 - player.y * 32.0) * 128.0) as i16,
//                                 0,
//                             ));
//                             connection
//                                 .write_packet(OutgoingPacket::SynchronizePlayerPosition {
//                                     x: player.x,
//                                     y,
//                                     z: player.z,
//                                     yaw: player.yaw,
//                                     pitch: player.pitch,
//                                     flags: 0,
//                                     teleport_id: 0,
//                                 })
//                                 .await;
//                             last_timeout = Instant::now();
//                         }
//                         KeepAlive { .. } => {
//                             // debug!("Keep alive received");
//                             last_timeout = Instant::now();
//                         }
//                         SetPlayerPosition { .. } => {}
//                         SetPlayerPositionAndRotation {
//                             x,
//                             y,
//                             z,
//                             mut yaw,
//                             pitch,
//                             on_ground,
//                         } => {
//                             events.lock().await.push(Event::PlayerMove(
//                                 entity_id,
//                                 ((x * 32.0 - player.x * 32.0) * 128.0) as i16,
//                                 ((y * 32.0 - player.y * 32.0) * 128.0) as i16,
//                                 ((z * 32.0 - player.z * 32.0) * 128.0) as i16,
//                             ));

//                             moved = true;
//                             yaw = yaw.rem_euclid(360.0);
//                             if yaw > 180.0 {
//                                 yaw -= 360.0;
//                             }
//                             (
//                                 player.x,
//                                 player.y,
//                                 player.z,
//                                 player.yaw,
//                                 player.pitch,
//                                 player.on_ground,
//                             ) = (x, y, z, yaw, pitch, on_ground);
//                             player_data.lock().await.insert(entity_id, player.clone());
//                         }
//                         SetPlayerRotation {
//                             mut yaw,
//                             pitch,
//                             on_ground,
//                         } => {
//                             events
//                                 .lock()
//                                 .await
//                                 .push(Event::PlayerMove(entity_id, 0, 0, 0));

//                             yaw = yaw.rem_euclid(360.0);
//                             if yaw > 180.0 {
//                                 yaw -= 360.0;
//                             }

//                             (player.yaw, player.pitch, player.on_ground) = (yaw, pitch, on_ground);
//                             player_data.lock().await.insert(entity_id, player.clone());
//                         }
//                         SetPlayerOnGround { on_ground } => {
//                             events
//                                 .lock()
//                                 .await
//                                 .push(Event::PlayerMove(entity_id, 0, 0, 0));

//                             player.on_ground = on_ground;
//                             player_data.lock().await.insert(entity_id, player.clone());
//                         }
//                     }
//                 }
//             }
//             Err(_) => (),
//         }

//         // if playstate_tick % 1_000_000 == 0 {
//         //     let block: (i32, i32, i32) = player.get_block();
//         //     let chunk: (i32, i32) = player.get_chunk();
//         //     let chunk_data: MutexGuard<'_, HashMap<(i32, i32), Chunk>> = chunk_data.lock().await;
//         //     debug!(
//         //         "Highest block at {:?} is y={:?}",
//         //         block,
//         //         chunk_data
//         //             .get(&chunk)
//         //             .unwrap()
//         //             .max_height_at(block.0, block.1)
//         //     );
//         // }

//         if moved {
//             let new_center_chunk: (i32, i32) = (player.x as i32 / 16, player.z as i32 / 16);

//             connection
//                 .write_packet(OutgoingPacket::SetCenterChunk {
//                     chunk_x: new_center_chunk.0,
//                     chunk_z: new_center_chunk.1,
//                 })
//                 .await;

//             let mut chunk_data: MutexGuard<'_, HashMap<(i32, i32), Chunk>> =
//                 chunk_data.lock().await;

//             for x in -16 + new_center_chunk.0..=16 + new_center_chunk.0 {
//                 for z in -16 + new_center_chunk.1..=16 + new_center_chunk.1 {
//                     if !((-16 + center_chunk.0..=16 + center_chunk.0).contains(&x)
//                         && (-16 + center_chunk.1..=16 + center_chunk.1).contains(&z))
//                     {
//                         connection
//                             .write_packet(OutgoingPacket::ChunkDataAndUpdateLight {
//                                 chunk_x: x,
//                                 chunk_z: z,
//                                 heightmaps: nbt!({}),
//                                 data: chunk_data.get(&(x, z)).cloned().unwrap_or_else(|| {
//                                     let chunk: Chunk = generate_chunk(x, z);
//                                     chunk_data.insert((x, z), chunk.clone());
//                                     chunk
//                                 }),
//                                 block_entities: vec![],
//                                 sky_light_mask: vec![],
//                                 block_light_mask: vec![],
//                                 empty_sky_light_mask: vec![],
//                                 empty_block_light_mask: vec![],
//                                 sky_light_arrays: vec![],
//                                 block_light_arrays: vec![],
//                             })
//                             .await;
//                         last_timeout = Instant::now();
//                     }
//                 }
//             }

//             center_chunk = new_center_chunk;
//         }

//         // Handle events
//         let mut packets: Vec<OutgoingPacket> = vec![];
//         if let Some(event) = events.lock().await.iter().nth(event_index) {
//             event_index += 1;

//             use Event::*;
//             use OutgoingPacket::*;
//             match event {
//                 PlayerJoin(eid) if *eid != entity_id => {
//                     let pd: MutexGuard<'_, HashMap<i32, Player>> = player_data.lock().await;
//                     let p: Option<&Player> = pd.get(eid);
//                     if let Some(p) = p {
//                         use PlayerActions::*;
//                         packets.push(PlayerInfoUpdate {
//                             actions: 0x01 | 0x08,
//                             players: vec![(
//                                 Uuid::offline(p.name.to_string()),
//                                 vec![
//                                     AddPlayer {
//                                         name: p.name.to_string(),
//                                         properties: vec![],
//                                     },
//                                     UpdateListed { listed: true },
//                                 ],
//                             )],
//                         });
//                         packets.push(SpawnPlayer {
//                             entity_id: p.id,
//                             player_uuid: Uuid::offline(p.name.to_string()),
//                             x: p.x,
//                             y: p.y,
//                             z: p.z,
//                             yaw: p.yaw,
//                             pitch: p.pitch,
//                         });
//                     }
//                 }
//                 PlayerMove(eid, dx, dy, dz) if *eid != entity_id => {
//                     if let Some(p) = player_data.lock().await.get(eid) {
//                         packets.push(UpdateEntityPositionAndRotation {
//                             entity_id: p.id,
//                             dx: *dx,
//                             dy: *dy,
//                             dz: *dz,
//                             yaw: p.yaw,
//                             pitch: p.pitch,
//                             on_ground: p.on_ground,
//                         });
//                         packets.push(SetHeadRotation {
//                             entity_id: p.id,
//                             head_yaw: p.yaw,
//                         });
//                     }
//                 }
//                 PlayerQuit(eid) if *eid != entity_id => {
//                     packets.push(RemoveEntities {
//                         entity_ids: vec![*eid],
//                     });
//                 }
//                 _ => {}
//             }
//         }

//         connection.write_packets(packets).await;

//         tick_times.push(start.elapsed());
//     }

//     events.lock().await.push(Event::PlayerQuit(entity_id));
//     entity_ids
//         .lock()
//         .await
//         .retain(|eid: &i32| *eid != entity_id);
//     player_data.lock().await.remove(&entity_id);
//     if state == ConnectionState::Play {
//         config.lock().await.status.players.online -= 1;
//     }

//     debug!("Connection with {} closed", player.name);
//     debug!(
//         "Average tick time: {:?}μs",
//         tick_times.iter().map(|d| d.as_micros()).sum::<u128>() as f64 / tick_times.len() as f64
//     );
// }
