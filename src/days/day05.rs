fn get_updates(lines: Vec<String>,invalid_updates: bool) -> Vec<Vec<i32>> {
    let mut ordering_rule = Vec::<(i32, i32)>::new();
    let mut update_pages = Vec::<Vec<i32>>::new();
    let mut in_the_updates_section = false;
    
    let mut updates_that_apply = Vec::<Vec<i32>>::new();
    let mut updates_that_need_fix = Vec::<Vec<i32>>::new();
    
    for line in lines {
        if line.is_empty() {
            in_the_updates_section = true;
        } else if in_the_updates_section {
            let list_of_pages = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            update_pages.push(list_of_pages);
        } else {
            let parts = line.split("|").collect::<Vec<&str>>();
            ordering_rule.push((parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap()));
        }
    }
    
    for update_rule in update_pages {
        let mut valid = true;
        for rule in &ordering_rule {
            let left = match update_rule.iter().position(|&r| r == rule.0){
                Some(x) => x,
                None => continue
            };
            let right = match update_rule.iter().position(|&r| r == rule.1){
                Some(x) => x,
                None => continue
            };
            
            if right < left {
                valid = false;
                break;
            }
        }
        
        if valid {
            updates_that_apply.push(update_rule);
        } else {
            updates_that_need_fix.push(update_rule);
        }
    }
    
    if invalid_updates {
        updates_that_need_fix
    } else {
        updates_that_apply
    }
}

fn fix_updates(update: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    update
}

pub fn part1(lines: Vec<String>) -> String {
    
    let updates_that_apply = get_updates(lines, false);

    let mut total = 0;
    
    for update in updates_that_apply {
        
        let half_index = update.len() / 2;
        total += update[half_index];
        
    }
    
    total.to_string()
}

pub fn part2(lines: Vec<String>) -> String {

    let updates_that_need_fix = get_updates(lines, true);

    let mut total = 0;
    
    for update in updates_that_need_fix {
        
        let half_index = update.len() / 2;
        total += update[half_index];
        
    }
    
    total.to_string()
}