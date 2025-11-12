use std::{cmp::Ordering};

struct Data {
    ordering_rules: Vec<(i32, i32)>,
    update_pages: Vec<Vec<i32>>
}

impl Data {
    fn new(ordering_rules: Vec<(i32, i32)>, update_pages: Vec<Vec<i32>>) -> Data {
        Data {
            ordering_rules: ordering_rules,
            update_pages: update_pages
        }
    }
}

fn get_data(lines: Vec<String>) -> Data {
    let mut ordering_rules = Vec::<(i32, i32)>::new();
    let mut update_pages = Vec::<Vec<i32>>::new();
    let mut in_the_updates_section = false;
    
    for line in lines {
        if line.is_empty() {
            in_the_updates_section = true;
        } else if in_the_updates_section {
            let list_of_pages = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            update_pages.push(list_of_pages);
        } else {
            let parts = line.split("|").collect::<Vec<&str>>();
            ordering_rules.push((parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap()));
        }
    }
    
    Data::new(ordering_rules, update_pages)
}

fn get_updates(data: &Data, invalid_updates: bool) -> Vec<Vec<i32>> {

    let ordering_rules = &data.ordering_rules;
    let update_pages = &data.update_pages;

    let mut updates_that_apply = Vec::<Vec<i32>>::new();
    let mut updates_that_need_fix = Vec::<Vec<i32>>::new();
    
    for update_rule in update_pages {
        let mut valid = true;
        for rule in ordering_rules {
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
            updates_that_apply.push(update_rule.clone());
        } else {
            updates_that_need_fix.push(update_rule.clone());
        }
    }
    
    if invalid_updates {
        updates_that_need_fix
    } else {
        updates_that_apply
    }
}

fn fix_updates(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {

    let mut ordered_update = update.clone();

    let compare = |x: &i32, y: &i32| {
        let (x, y) = (*x, *y);
        if rules.contains(&(x, y)) {
            Ordering::Less
        } else if rules.contains(&(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    ordered_update.sort_by(compare);

    ordered_update
}

fn get_total_sum_of_middle_index(updates_that_apply: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    
    for update in updates_that_apply {
    
        let half_index = update.len() / 2;
        total += update[half_index];
    
    }
    total
}

pub fn part1(lines: Vec<String>) -> String {

    let data = get_data(lines);
    
    let updates_that_apply = get_updates(&data, false);

    let total = get_total_sum_of_middle_index(updates_that_apply);
    
    total.to_string()
}

pub fn part2(lines: Vec<String>) -> String {

    let data = get_data(lines);
    
    let updates_that_need_fix = get_updates(&data, true);
    let mut fixed_updates = Vec::<Vec<i32>>::new();
    
    for update in updates_that_need_fix {

        let rules = data.ordering_rules.clone();

        let fixed_update =  fix_updates(&update, &rules);

        fixed_updates.push(fixed_update);
        
    }

    let total = get_total_sum_of_middle_index(fixed_updates);
    
    total.to_string()
}