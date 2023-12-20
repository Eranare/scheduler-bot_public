//Generate actual image in here for the timetable which will show weekly days. from 13.00 to 23.00.
//when 4 users have reacted to something as available show that as a green block. 

// file path to image: ./resources/images/timetable.png
use image::{Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use crate::Error;
use crate::DbPool;
use std::collections::HashSet;

pub async fn generate_timetable_image(db_pool: &DbPool, guild_id: i64) -> Result<String, Error> {
    let conn = db_pool.get().await.map_err(|e| Box::new(e))?;

    // SQL query to get unavailable time slots
    let sql = "SELECT day, COUNT(*) 
    FROM user_time_slots 
    INNER JOIN users ON user_time_slots.user_record_id = users.id
    WHERE users.guild_id = $1 AND user_time_slots.available = false
    GROUP BY day;";
    let rows = conn.query(sql, &[&guild_id]).await.map_err(|e| Box::new(e))?;

    // Load the base timetable image
    let mut timetable_image = image::open("/usr/local/bin/resources/images/timetable.png")
    .map_err(|e| Box::new(e))?.to_rgba8();

    // Create a set of unavailable timeslots
    let mut unavailable_slots: HashSet<String> = HashSet::new();
    for row in rows {
        let day: String = row.get(0);
        unavailable_slots.insert(day);
    }

    // Process each timeslot and apply the color
    for timeslot in TIMESLOTS.iter() {
        let coords = get_time_slot_coordinates(timeslot);
        let color = if unavailable_slots.contains(*timeslot) {
            Rgba([255, 0, 0, 255]) // Red for unavailable
        } else {
            Rgba([0, 255, 0, 255]) // Green for available
        };

        // Apply the color to the coordinates
        for coord in coords {
            draw_filled_rect_mut(&mut timetable_image, Rect::at(coord.x, coord.y).of_size(coord.width, coord.height), color);
        }
    }

    // Save the updated image
    let file_name = format!("timetable_{}.png", guild_id);
    let file_path = format!("/usr/local/bin/resources/images/{}", file_name);
    timetable_image.save(file_path.clone()).map_err(|e| Box::new(e))?;

    Ok(file_path)
}   
    //image::open("./resources/images/timetable.png")

    //let file_path = format!("./resources/images/{}", file_name);

// Define a structure to hold coordinates for a timeslot
struct TimeSlotCoordinates {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

const TIMESLOTS: [&str; 14] = [
    "Monday 2-5 PM", "Monday 7-10 PM", 
    "Tuesday 2-5 PM", "Tuesday 7-10 PM", 
    "Wednesday 2-5 PM", "Wednesday 7-10 PM", 
    "Thursday 2-5 PM", "Thursday 7-10 PM", 
    "Friday 2-5 PM", "Friday 7-10 PM", 
    "Saturday 2-5 PM", "Saturday 7-10 PM", 
    "Sunday 2-5 PM", "Sunday 7-10 PM",
];


fn get_time_slot_coordinates(day: &str) -> Vec<TimeSlotCoordinates> {
    match day {
        "Monday 2-5 PM" => vec![TimeSlotCoordinates { x: 100, y: 41, width: 100, height: 86 }],
        "Monday 7-10 PM" => vec![TimeSlotCoordinates { x: 100, y: 147, width: 100, height: 83 }],
        "Tuesday 2-5 PM" => vec![TimeSlotCoordinates { x: 201, y: 41, width: 101, height: 86 }],
        "Tuesday 7-10 PM" => vec![TimeSlotCoordinates { x: 201, y: 147, width: 101, height: 83 }],
        "Wednesday 2-5 PM" => vec![TimeSlotCoordinates { x: 303, y: 41, width: 101, height: 86 }],
        "Wednesday 7-10 PM" => vec![TimeSlotCoordinates { x: 303, y: 147, width: 101, height: 83 }],
        "Thursday 2-5 PM" => vec![TimeSlotCoordinates { x: 405, y: 41, width: 101, height: 86 }],
        "Thursday 7-10 PM" => vec![TimeSlotCoordinates { x: 405, y: 147, width: 101, height: 83 }],
        "Friday 2-5 PM" => vec![TimeSlotCoordinates { x: 507, y: 41, width: 101, height: 86 }],
        "Friday 7-10 PM" => vec![TimeSlotCoordinates { x: 507, y: 147, width: 101, height: 83 }],
        "Saturday 2-5 PM" => vec![TimeSlotCoordinates { x: 609, y: 41, width: 101, height: 86 }],
        "Saturday 7-10 PM" => vec![TimeSlotCoordinates { x: 609, y: 147, width: 101, height: 83 }],
        "Sunday 2-5 PM" => vec![TimeSlotCoordinates { x: 711, y: 41, width: 101, height: 86 }],
        "Sunday 7-10 PM" => vec![TimeSlotCoordinates { x: 711, y: 147, width: 101, height: 83 }],
        _ => vec![],
    }
}

/*
//image coordinates in pixels per time slot.

// Monday
// 2-5 PM: 100-41 to 200-127
// 7-10 PM: 100-147 to 200-230

// Tuesday
// 2-5 PM: 201-41 to 302-127
// 7-10 PM: 201-147 to 302-230

// Wednesday
// 2-5 PM: 303-41 to 404-127
// 7-10 PM: 303-147 to 404-230

// Thursday
// 2-5 PM: 405-41 to 506-127
// 7-10 PM: 405-147 to 506-230

// Friday
// 2-5 PM: 507-41 to 608-127
// 7-10 PM: 507-147 to 608-230

// Saturday
// 2-5 PM: 609-41 to 710-127
// 7-10 PM: 609-147 to 710-230

// Sunday
// 2-5 PM: 711-41 to 812-127
// 7-10 PM: 711-147 to 812-230

*/