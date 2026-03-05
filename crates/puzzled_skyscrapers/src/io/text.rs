// impl TxtPuzzle<SkyscraperState> for Skyscrapers {
//     fn read_text(reader: &mut read::TxtState) -> read::Result<(Self, SkyscraperState)> {
//         let ((cells, entries), sides) = reader.read_sided_cells_and_entries()?;
//
//         eprintln!("2");
//         let (metadata, timer) = reader.read_metadata(None)?;
//
//         // Create state
//         eprintln!("3");
//         let solutions = cells.map_ref(|cell| cell.solution);
//         let timer = timer.unwrap_or_default();
//         let state = SkyscraperState::new(solutions, entries, timer);
//
//         // Create clues
//         let mut clues = Clues::new(BTreeMap::default(), cells.rows(), cells.cols());
//         let directions = [
//             Direction::Down,
//             Direction::Left,
//             Direction::Up,
//             Direction::Right,
//         ];
//
//         for (side, direction) in sides.into_iter().zip(directions) {
//             for (idx, h) in side.into_iter().enumerate() {
//                 let Some(height) = h else {
//                     continue;
//                 };
//
//                 let line = match direction {
//                     Direction::Up | Direction::Down => Line::Col(idx),
//                     _ => Line::Row(idx),
//                 };
//
//                 let id = (line, direction).into();
//                 clues.insert(id, *height);
//             }
//         }
//
//         // Create puzzle
//         let skyscrapers = Skyscrapers::new(cells, clues, metadata);
//
//         Ok((skyscrapers, state))
//     }
// }
