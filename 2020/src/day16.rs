use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

pub fn day16() -> Result<()> {
    let data = INPUT;

    let (rules, my_ticket, other_tickets) = parse_data(&data)?;
    let (mut bad_fields, mut valid_tickets) = (Vec::new(), Vec::new());
    other_tickets
        .iter() // iter over Tickets
        .map(|t| (t, invalid_fields(&rules, t)))
        .for_each(|(t, invalid_fields)| {
            if invalid_fields.is_empty() {
                // push good tickets into 'valid tickets'
                valid_tickets.push(t.clone());
            } else {
                // push fields from bad tickets into bad fields
                bad_fields.extend(invalid_fields);
            }
        });

    part1(&bad_fields)?;
    part2(&rules, my_ticket, &valid_tickets)?;
    Ok(())
}

type RuleSet = HashMap<String, (Range, Range)>;
type Range = (usize, usize);
type Ticket = Vec<usize>;

fn part1(bad_fields: &[usize]) -> Result<()> {
    println!("AoC2019 16.1 -> {}", bad_fields.iter().sum::<usize>());
    Ok(())
}

fn part2(rules: &RuleSet, my: Ticket, tickets: &[Ticket]) -> Result<()> {
    let product_of_my_departure_columns: usize = my
        .iter()
        .zip(find_rule_order(&rules, &tickets).iter())
        .filter(|(_v, name)| name.contains("departure"))
        .map(|(v, _)| v)
        .product();
    println!("2020 16.2 -> {}", product_of_my_departure_columns);
    Ok(())
}

fn find_rule_order(rules: &RuleSet, tickets: &[Ticket]) -> Vec<String> {
    let keys: HashSet<&String> = rules.keys().collect();
    let mut possible: Vec<_> = (0..keys.len()).map(|_| keys.clone()).collect();

    for ticket in tickets {
        for (i, value) in ticket.iter().enumerate() {
            let mut invalid_rule_names = Vec::new();
            for rule_name in &possible[i] {
                if !rule_matches(rules[*rule_name], *value) {
                    invalid_rule_names.push(*rule_name);
                }
            }
            for name in invalid_rule_names {
                possible[i].remove(name);
            }
        }
    }

    let mut processed_rules = Vec::new();
    loop {
        let has_1 = possible
            .iter()
            .enumerate()
            .filter(|(i, x)| x.len() == 1 && !processed_rules.contains(i))
            .map(|(i, x)| (i, x.iter().take(1).collect::<Vec<_>>()[0].to_string()))
            .next()
            .unwrap();
        processed_rules.push(has_1.0);
        for (i, item) in possible.iter_mut().enumerate() {
            if i == has_1.0 {
                continue;
            } else {
                item.remove(&has_1.1);
            }
        }
        if possible.iter().all(|x| x.len() == 1) {
            break;
        }
    }

    // unwrap should be safe, as we loop above until all columns have a single
    // associated rule
    possible
        .iter()
        .map(|x| x.iter().next().unwrap().to_string())
        .collect()
}

#[allow(clippy::ptr_arg)]
fn invalid_fields(rs: &RuleSet, t: &Ticket) -> Vec<usize> {
    t.iter()
        .filter(|v| !rs.values().any(|&rule| rule_matches(rule, **v)))
        .copied()
        .collect()
}

fn parse_rule(s: &str) -> Result<(String, (Range, Range))> {
    let mut parts = s.split(':');
    let name = parts.next().ok_or_else(|| anyhow!("No name for rule"))?;
    let ranges = parts
        .next()
        .ok_or_else(|| anyhow!("No ranges for rule"))?
        .split(" or ")
        .map(|x| {
            x.trim()
                .split('-')
                .map(|y| y.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    // let mut values = Vec::new();
    // for range in ranges {
    //     values.push((range[0], range[1]));
    // }
    Ok((
        name.to_string(),
        ((ranges[0][0], ranges[0][1]), (ranges[1][0], ranges[1][1])),
    ))
}

fn parse_ticket(s: &str) -> Vec<usize> {
    s.split(',').filter_map(|x| Some(x.parse().ok()?)).collect()
}

fn parse_data(data: &str) -> Result<(RuleSet, Ticket, Vec<Ticket>)> {
    let mut rules: HashMap<String, (Range, Range)> = HashMap::new();
    let mut my_ticket: Ticket = Vec::new();
    let mut other_tickets: Vec<Ticket> = Vec::new();
    let mut sections = data.split("\n\n").map(|sect| sect.lines());

    match sections.next() {
        Some(rule_text) => {
            for line in rule_text {
                let (name, values) = parse_rule(line)?;
                rules.insert(name, values);
            }
        }
        None => return Err(anyhow!("No rule text in input")),
    }

    match sections.next() {
        Some(my_ticket_text) => {
            for line in my_ticket_text {
                my_ticket = parse_ticket(line);
            }
        }
        None => return Err(anyhow!("No ticket for me in input")),
    }

    match sections.next() {
        Some(others_ticket_text) => {
            for line in others_ticket_text {
                other_tickets.push(parse_ticket(line));
            }
        }
        None => return Err(anyhow!("No tickets for others in input")),
    }

    Ok(((rules), my_ticket, other_tickets))
}

#[inline(always)]
fn rule_matches((lower_range, upper_range): (Range, Range), num: usize) -> bool {
    (lower_range.0..=lower_range.1)
        .chain(upper_range.0..=upper_range.1)
        .any(|x| x == num)
}

const INPUT: &str = "departure location: 45-422 or 444-950
departure station: 36-741 or 752-956
departure platform: 46-788 or 806-967
departure track: 46-57 or 70-950
departure date: 35-99 or 108-974
departure time: 42-883 or 903-962
arrival location: 47-83 or 95-953
arrival station: 31-227 or 240-970
arrival platform: 48-840 or 853-964
arrival track: 49-487 or 499-964
class: 33-363 or 381-959
duration: 35-509 or 522-951
price: 38-590 or 601-950
route: 41-266 or 285-962
row: 44-402 or 419-962
seat: 41-615 or 634-956
train: 47-156 or 178-954
type: 44-313 or 338-964
wagon: 30-110 or 133-970
zone: 38-541 or 550-965

your ticket:
109,199,223,179,97,227,197,151,73,79,211,181,71,139,53,149,137,191,83,193

nearby tickets:
916,733,486,481,857,191,77,374,684,717,145,568,53,390,390,772,471,780,187,458
941,613,92,946,358,133,873,670,866,461,608,95,650,484,482,948,944,480,356,654
456,150,198,912,710,851,781,350,767,767,663,564,451,726,210,570,874,787,701,940
220,394,552,263,865,612,453,906,856,686,739,776,765,223,534,289,371,557,536,882
934,207,813,637,563,401,447,224,761,720,785,836,913,853,141,156,814,217,993,777
466,737,988,584,73,244,226,866,682,485,934,499,666,729,806,56,684,249,148,736
719,555,659,482,410,391,457,531,835,502,70,476,340,708,467,941,761,708,384,99
447,649,463,752,459,768,636,643,253,739,197,650,580,917,369,839,584,643,52,396
266,300,645,919,736,944,759,676,929,936,78,475,917,487,976,679,243,585,696,929
907,733,919,355,692,474,339,180,767,535,569,560,651,98,234,681,676,929,475,684
566,753,532,867,741,881,550,577,482,935,670,640,862,581,716,261,21,218,815,863
251,730,719,855,109,180,945,706,736,197,308,764,755,94,929,352,382,313,155,925
55,118,75,296,303,222,135,715,645,602,298,828,769,181,385,679,944,782,825,643
398,303,655,292,911,697,760,861,487,139,451,522,353,938,706,229,551,528,580,110
248,207,53,715,757,469,859,140,853,532,814,458,636,401,833,160,636,763,944,391
837,457,555,249,294,95,878,811,784,338,182,758,916,226,752,576,396,760,433,71
453,237,257,388,556,253,194,156,582,214,607,922,833,709,78,571,348,778,560,470
352,975,472,299,151,247,612,285,458,662,197,500,210,814,693,143,346,56,740,384
603,876,939,736,192,330,196,541,820,508,838,304,904,249,153,387,349,693,300,193
688,381,509,79,771,179,396,467,856,711,672,858,774,806,390,935,652,678,90,728
241,538,588,999,579,78,522,585,262,142,381,663,194,302,213,868,248,469,526,703
707,587,253,74,839,342,147,711,901,911,562,776,480,147,300,946,485,381,811,151
536,576,459,155,258,915,881,873,82,257,239,382,344,360,615,223,470,660,757,941
50,485,607,449,644,786,205,452,345,83,485,110,184,408,865,299,654,556,246,719
866,98,825,643,702,471,881,603,741,603,671,585,860,256,657,103,346,907,916,930
142,926,70,386,861,618,768,388,719,529,784,612,226,566,672,459,465,558,582,830
720,770,136,346,537,863,250,459,545,460,647,479,563,777,483,394,650,577,718,615
691,138,80,452,141,289,301,708,903,865,925,979,881,420,674,210,693,565,868,291
3,556,557,468,924,242,932,671,178,215,675,924,386,677,143,566,98,178,448,733
654,259,568,392,544,601,572,785,299,767,575,638,659,932,603,651,474,258,419,609
662,476,718,584,637,713,663,344,446,938,461,602,929,836,204,68,577,809,50,920
686,198,290,298,524,722,95,305,568,570,503,813,80,699,559,227,653,834,898,394
560,535,259,147,753,644,760,534,370,74,362,614,540,222,576,137,456,469,207,294
660,815,740,293,881,545,210,862,74,723,529,218,659,388,467,908,739,662,392,96
289,925,355,451,686,141,344,909,910,487,765,355,708,622,110,194,260,880,605,304
666,146,386,783,18,474,683,940,819,936,680,567,254,735,818,883,918,645,56,658
912,682,786,778,733,711,215,420,719,109,468,502,203,866,78,556,737,375,359,741
509,265,811,257,334,199,713,201,399,471,945,754,223,363,558,615,99,95,295,728
480,607,5,930,389,859,783,609,640,589,298,601,739,571,451,872,560,859,941,474
813,921,552,754,924,384,727,817,565,262,24,394,653,473,914,769,730,606,718,144
144,713,313,468,979,454,389,81,719,304,467,910,769,181,484,706,735,765,643,765
738,309,360,784,198,682,193,551,247,863,785,689,734,154,358,430,241,240,785,53
312,548,470,218,186,363,356,95,832,649,250,864,445,585,464,447,555,208,135,585
451,586,922,503,363,395,928,867,938,906,193,776,458,154,578,375,535,558,249,837
133,563,761,879,784,905,919,718,241,810,879,526,663,783,776,655,665,703,149,993
467,920,879,569,64,693,468,51,833,531,453,72,602,726,922,875,227,879,782,464
669,383,349,939,581,484,10,540,653,470,145,815,473,245,385,753,861,535,192,777
466,536,357,731,501,720,342,506,344,191,196,460,587,691,146,448,468,94,217,98
483,306,853,137,684,311,680,194,395,946,696,660,775,206,74,859,815,811,860,368
689,698,110,560,774,759,78,571,133,820,246,989,669,266,459,286,908,739,910,857
240,156,931,636,790,460,528,141,729,509,185,813,211,727,694,681,710,924,221,190
311,709,278,532,453,875,77,388,927,729,870,540,645,136,685,723,667,50,785,700
559,251,544,862,529,612,530,866,463,861,719,360,736,714,307,148,788,476,667,351
458,103,781,98,771,302,905,97,470,582,668,220,150,722,654,358,774,209,826,422
56,179,155,386,933,81,454,7,382,673,217,646,351,723,758,540,71,687,419,639
148,788,193,784,922,388,670,929,315,553,660,346,701,346,825,75,693,99,527,79
306,946,784,754,782,353,526,656,552,582,562,645,171,227,530,865,312,501,451,476
655,452,288,222,762,188,500,565,527,213,870,645,603,548,879,389,363,752,556,463
836,934,986,222,71,522,552,264,473,389,52,726,420,206,707,667,730,354,457,482
189,456,482,642,447,248,346,527,523,301,533,295,232,738,771,756,574,573,452,700
943,675,486,392,255,366,647,180,853,203,341,470,922,382,144,205,906,481,906,583
226,293,653,180,584,487,768,209,467,823,189,821,187,129,421,666,651,530,927,538
472,313,663,856,939,196,188,210,437,179,293,461,648,533,302,529,712,242,294,342
637,390,481,295,470,146,920,17,499,834,687,740,478,144,463,875,341,483,309,356
665,528,941,740,210,292,552,302,258,145,992,460,294,940,57,474,944,693,605,643
290,691,449,860,283,946,187,560,503,602,858,811,574,781,704,303,527,647,340,227
668,183,54,823,691,878,720,590,808,732,643,523,3,482,80,719,259,140,213,74
610,730,733,392,257,452,854,694,467,528,827,853,189,56,11,863,570,420,729,677
878,477,399,244,55,471,574,347,714,643,196,945,389,650,900,730,708,259,190,711
563,60,453,481,660,472,572,709,553,923,877,768,655,201,711,859,206,788,400,294
387,226,560,208,401,775,135,725,421,948,911,930,297,269,582,265,916,80,808,565
566,660,817,468,448,723,758,713,731,637,487,484,694,144,669,665,57,8,718,307
734,681,758,677,460,164,347,691,699,587,860,359,691,182,812,291,343,677,773,313
671,133,264,698,679,862,145,509,72,677,508,780,615,69,183,343,910,634,870,71
385,252,667,195,165,227,205,652,397,864,200,780,821,740,571,717,924,342,530,149
566,692,683,720,576,810,838,605,635,464,770,555,485,363,21,246,648,824,467,304
562,563,567,453,346,182,336,731,343,635,473,97,700,153,206,400,386,294,859,348
343,585,344,261,820,18,726,468,651,82,666,606,152,924,460,109,763,500,508,645
201,871,675,400,756,736,676,942,758,506,937,555,573,934,444,385,247,282,357,241
775,355,698,265,704,86,302,934,564,834,921,361,421,446,137,644,719,565,832,729
181,4,582,858,855,254,54,918,558,775,470,611,146,478,401,560,522,917,352,264
777,245,224,307,540,313,348,688,467,995,483,56,929,734,685,576,189,221,723,866
860,588,830,664,692,579,148,668,688,263,220,346,108,738,225,156,727,740,233,762
140,139,561,875,754,501,204,585,711,939,605,144,712,75,625,95,772,926,657,500
308,572,73,945,586,112,485,930,917,202,214,721,761,722,400,505,400,263,708,465
477,81,167,446,220,288,150,472,785,603,947,51,200,813,615,240,881,179,582,915
265,925,227,700,259,643,243,445,715,759,717,585,719,389,71,312,342,172,357,725
143,896,295,728,218,917,351,258,483,566,501,650,578,204,198,138,685,258,156,344
245,778,590,660,186,499,451,782,83,294,549,785,487,705,240,508,98,295,636,310
522,700,83,705,295,450,401,637,570,507,270,559,422,680,466,709,140,384,155,472
470,586,924,451,876,360,508,540,647,259,679,190,666,375,684,676,928,470,156,604
731,684,754,769,186,915,809,359,392,753,525,291,308,306,344,796,540,555,450,938
695,362,528,658,343,439,191,638,569,479,737,853,573,911,389,295,815,659,338,189
78,787,454,565,865,246,248,55,219,15,601,222,148,815,923,853,604,634,340,148
635,384,645,687,985,349,341,383,97,756,686,866,691,340,835,360,606,382,70,782
584,783,219,117,206,777,288,762,343,834,681,136,141,298,608,567,312,685,357,934
741,179,361,172,257,304,947,449,80,211,769,201,646,286,148,210,761,755,649,246
183,742,661,946,52,508,762,532,766,572,818,138,664,464,758,79,698,754,672,537
939,565,319,839,224,740,530,924,862,452,526,142,523,248,76,723,78,349,206,825
54,182,357,821,135,205,859,143,832,304,535,726,637,215,668,568,55,823,58,216
931,57,847,642,348,449,222,653,760,839,645,580,664,709,255,534,948,667,932,709
216,99,766,156,286,483,242,400,848,455,226,605,240,289,395,392,154,610,221,202
541,526,606,480,751,943,782,736,761,294,648,754,500,138,56,906,938,853,767,96
551,741,820,205,879,774,903,218,198,98,71,907,538,351,701,244,301,835,336,668
138,783,865,392,400,929,948,907,485,82,732,720,82,467,869,813,947,699,277,766
250,354,199,728,735,285,590,339,70,819,648,460,352,309,579,888,532,503,95,774
384,628,763,82,605,683,603,140,214,649,537,673,304,914,451,823,661,305,825,70
398,814,198,257,772,274,586,354,449,259,534,213,777,484,194,179,677,809,661,613
461,297,500,361,298,765,673,240,718,202,719,464,307,903,83,91,605,351,220,678
806,400,737,834,662,356,219,824,739,310,224,683,480,761,363,586,139,421,224,128
947,153,111,74,250,241,817,501,606,575,930,863,862,57,289,944,922,757,879,209
614,792,386,910,836,583,360,872,561,391,574,645,357,187,719,532,680,83,390,607
635,343,649,156,688,912,808,214,307,136,693,727,550,309,530,751,208,143,726,384
646,395,481,766,853,461,264,681,620,468,687,381,806,602,529,251,554,837,830,421
918,752,627,652,478,941,363,689,723,257,755,758,72,254,287,761,764,854,575,300
924,450,876,915,656,289,708,614,119,503,717,552,774,144,698,266,876,682,806,703
536,571,155,251,831,207,354,97,635,762,581,821,680,95,545,188,401,776,55,221
357,476,187,729,186,913,926,455,534,639,215,371,670,151,153,362,357,610,664,206
383,400,347,57,567,298,501,830,869,732,448,565,402,855,240,281,704,186,812,937
821,241,663,925,296,780,702,613,825,363,76,655,946,445,927,459,721,736,616,558
188,267,508,357,454,908,724,73,574,825,765,554,398,812,639,534,819,467,291,725
477,73,670,302,812,785,721,656,285,474,196,708,285,334,868,662,487,186,505,865
96,699,577,484,385,307,466,211,924,740,671,586,774,613,948,222,226,109,102,262
905,678,183,682,468,227,631,740,780,649,216,455,306,199,153,766,357,298,560,474
222,722,905,82,933,311,339,471,946,385,720,928,245,800,806,198,109,776,308,76
781,485,294,504,821,246,339,788,298,771,77,836,589,356,754,687,857,977,297,709
73,151,610,564,831,100,610,644,579,156,133,191,185,926,482,679,96,647,659,634
817,676,539,907,878,298,655,258,534,540,505,246,382,255,363,543,257,134,873,670
861,928,245,296,195,775,724,92,241,581,580,307,487,216,605,83,646,561,209,487
139,562,134,155,136,814,306,561,606,653,859,539,481,144,314,578,57,610,508,712
699,834,338,225,897,444,690,764,244,353,313,664,828,726,658,245,654,505,836,204
831,296,642,89,763,867,587,878,720,732,612,475,903,509,635,869,139,769,340,727
999,696,180,603,604,840,563,399,907,445,339,827,865,257,449,227,691,637,912,242
352,266,135,214,22,875,97,480,922,945,222,770,210,566,613,254,763,186,387,200
94,148,395,109,708,391,769,770,566,580,679,577,313,834,864,662,485,480,554,678
387,690,685,466,285,827,88,948,883,613,309,76,348,299,832,470,915,453,570,731
605,445,211,461,628,773,807,213,605,258,931,136,782,302,943,675,733,245,684,604
482,711,672,264,528,586,877,819,499,355,611,246,783,530,740,495,813,858,638,151
613,452,354,136,540,231,764,56,144,912,907,915,643,941,714,151,779,487,709,635
52,646,835,934,757,301,719,450,839,387,602,692,626,724,351,833,577,241,935,466
221,607,651,611,343,773,806,304,607,178,923,828,757,712,870,720,737,526,744,70
210,670,862,783,861,573,295,686,647,931,733,91,551,880,713,310,576,637,724,931
941,137,220,679,713,250,384,210,612,758,561,643,774,635,306,568,555,331,541,522
981,219,155,505,925,245,524,244,396,471,212,947,914,905,558,82,723,922,692,524
287,340,179,309,874,747,72,256,214,728,825,691,98,817,835,705,301,667,580,221
923,565,880,339,650,265,949,689,402,590,369,180,821,206,96,565,399,250,610,564
470,556,448,524,105,568,723,731,210,658,945,685,825,245,448,473,523,256,692,382
756,141,539,837,555,400,206,450,241,257,676,816,638,540,990,399,289,907,504,771
641,659,715,729,589,503,762,697,721,838,499,526,304,232,499,872,669,853,525,420
303,943,140,459,212,721,110,468,272,218,504,108,293,482,141,398,451,646,684,445
150,867,638,665,919,806,50,914,766,224,636,633,454,713,391,56,614,695,73,74
507,991,385,262,227,931,467,186,663,910,920,697,185,769,256,395,581,754,780,863
526,383,197,355,236,700,761,482,667,770,559,197,711,759,480,808,580,718,603,389
703,381,343,693,904,524,356,96,72,343,757,465,367,692,463,185,660,779,474,393
909,198,445,709,613,256,769,80,576,476,646,362,386,729,650,679,212,856,911,987
12,649,771,830,240,253,299,680,939,203,540,909,613,560,470,691,248,482,725,308
826,588,666,108,714,294,608,903,344,110,854,198,940,832,841,251,711,522,737,836
458,737,686,484,254,139,230,508,736,343,777,53,939,526,686,181,395,73,722,401
717,774,580,78,605,857,737,703,109,201,768,483,206,866,393,655,557,447,447,330
214,729,472,190,584,817,226,110,636,945,525,458,186,424,397,503,359,454,725,345
921,879,684,739,463,468,711,698,222,769,753,864,287,401,584,876,146,601,547,388
923,865,919,947,533,99,54,578,637,766,649,12,226,51,153,806,553,444,347,723
398,160,676,713,457,783,554,466,634,650,532,713,699,143,301,771,763,737,249,215
264,301,367,484,99,383,826,754,741,194,288,151,446,855,286,871,857,179,761,75
347,377,832,474,344,653,642,905,522,756,864,188,706,650,475,359,727,451,358,916
254,327,564,944,766,811,730,298,387,99,569,665,399,73,311,504,817,138,668,263
708,152,601,529,929,536,690,198,606,192,632,210,815,735,607,687,154,832,675,289
939,242,361,224,814,615,703,184,652,702,327,585,782,880,356,214,345,605,830,763
685,673,445,683,883,636,197,924,930,486,190,344,139,54,271,394,145,301,564,134
576,526,875,583,918,50,808,943,351,689,215,580,140,725,920,243,451,549,761,301
345,50,192,827,57,266,163,656,212,662,930,663,96,636,381,211,859,635,190,450
355,810,777,854,653,108,528,225,903,146,947,588,362,296,123,871,398,449,534,445
52,691,192,457,382,639,419,148,349,604,853,317,287,677,676,639,857,879,253,557
779,218,856,205,849,645,568,921,837,682,391,551,809,839,673,471,590,478,866,883
476,848,339,184,464,153,940,300,866,559,868,382,289,196,710,142,527,110,469,138
108,504,701,481,83,192,531,717,295,468,389,807,538,814,181,125,252,872,150,186
294,873,675,360,937,646,534,288,353,507,871,649,582,105,940,712,836,462,817,484
566,462,152,602,834,383,873,732,715,1,654,561,772,77,457,653,784,244,476,642
705,712,468,701,665,928,200,583,312,570,587,764,864,820,746,75,767,304,601,603
574,484,806,201,344,70,133,561,92,611,768,680,830,722,787,245,731,601,695,777
306,358,738,253,699,634,874,250,291,82,72,54,94,740,582,866,705,313,662,346
537,340,564,383,679,533,652,469,535,571,866,361,456,203,506,852,602,82,500,458
359,340,671,227,530,567,757,706,634,51,287,53,151,246,94,226,363,419,220,76
396,471,557,723,368,462,531,927,486,465,182,344,210,883,52,381,522,578,765,75
817,82,197,816,393,921,83,161,833,752,504,785,775,781,586,920,872,808,810,313
261,97,814,82,79,99,926,391,663,506,159,470,674,905,666,677,444,908,359,341
756,99,247,821,934,838,531,761,676,342,529,119,504,350,397,700,657,610,345,342
80,903,293,676,54,810,142,225,306,312,578,505,636,910,934,316,879,150,452,767
287,344,241,529,882,395,833,659,344,715,536,75,9,838,218,54,738,74,253,940
684,211,945,341,814,933,356,869,996,940,82,923,421,258,679,637,682,737,614,194
351,523,830,823,676,807,528,391,74,536,84,450,343,299,552,358,934,913,657,243
680,912,943,679,533,571,859,914,610,682,818,245,480,506,757,740,89,195,193,396
838,706,927,464,478,690,784,928,704,915,248,581,806,843,881,301,580,154,201,768
727,143,574,809,937,344,311,580,454,455,655,868,880,656,826,941,110,244,380,400
486,313,248,883,535,56,378,700,381,756,208,582,929,927,148,638,145,552,461,652
786,921,208,864,817,57,709,144,867,922,810,311,188,76,770,640,8,921,907,766
862,351,226,840,135,755,305,465,226,22,288,680,724,387,194,933,75,670,686,882
57,721,655,785,73,570,586,382,483,857,728,664,582,834,928,288,550,813,851,144
200,452,666,193,614,532,126,338,456,837,819,788,569,550,561,525,685,462,458,711
565,313,815,918,491,818,695,933,679,731,692,704,260,473,604,56,250,870,905,716
530,91,138,507,738,924,603,468,819,776,355,814,812,260,862,863,384,402,178,154
362,613,824,829,577,854,294,923,927,586,571,778,285,172,635,938,882,871,691,946
258,459,587,205,140,689,262,458,783,859,906,468,785,715,341,313,70,111,580,759
342,856,522,705,579,723,308,915,777,503,913,263,559,858,467,407,289,724,77,944
938,873,828,133,219,188,654,456,934,212,145,839,650,892,753,785,938,485,771,578
502,456,230,738,904,874,701,812,485,949,727,739,820,144,581,770,57,447,757,297
872,502,638,882,219,639,762,263,757,458,298,817,608,613,610,16,676,942,448,257
526,931,216,193,919,828,681,928,916,241,667,502,547,815,110,57,353,653,669,485
612,457,931,181,307,860,73,309,166,244,608,198,289,879,905,183,359,179,759,245
456,605,397,784,481,299,582,57,484,527,696,485,870,361,423,710,771,690,388,391
697,208,356,808,460,363,389,52,911,550,558,586,144,399,905,299,227,603,419,169
867,602,720,201,768,635,752,874,755,774,759,559,356,223,163,690,685,575,882,938
178,523,480,808,206,703,652,155,239,859,922,296,522,673,948,227,468,399,860,706
146,927,718,929,673,652,468,581,218,861,83,928,204,472,673,98,584,626,209,646
339,722,613,197,97,602,724,765,155,688,202,339,315,346,738,348,353,363,807,831
767,736,611,247,730,200,214,905,189,397,73,613,303,764,523,153,266,533,118,856
153,386,504,920,541,447,727,445,72,946,533,687,730,204,393,486,785,716,718,86
930,695,925,550,775,821,991,453,460,249,690,196,834,637,677,715,666,393,637,71
935,250,666,294,349,698,387,733,398,77,242,507,288,309,187,784,387,532,161,760
579,563,926,922,665,715,785,206,219,911,71,537,601,531,388,776,419,231,788,861
51,397,878,300,552,152,181,133,469,220,645,302,305,650,301,630,728,182,562,469
151,777,691,580,519,763,381,830,707,221,74,696,755,530,731,189,505,264,340,782
764,360,499,579,88,556,191,565,73,585,734,936,824,448,639,53,656,881,565,154
339,537,469,251,22,142,303,926,504,946,476,829,832,151,200,188,668,221,909,243
71,137,82,378,868,715,588,603,678,240,552,651,561,684,259,309,606,300,601,263
186,452,54,837,768,663,216,286,674,80,894,244,98,720,506,823,741,658,199,690
462,241,918,73,947,137,390,507,806,722,219,310,99,673,585,248,21,527,338,259
218,460,774,691,652,144,185,732,399,539,647,480,608,745,529,531,481,504,649,648
766,778,927,460,119,538,729,916,757,602,832,604,681,71,204,587,133,179,808,560
451,556,191,682,815,823,96,241,882,475,127,720,938,903,224,445,459,523,574,455
395,837,289,738,426,477,348,96,602,394,580,95,578,603,660,604,133,422,703,940
833,939,862,509,572,482,947,154,645,73,852,740,838,818,213,728,869,179,644,205
774,437,583,920,248,578,242,138,400,573,782,288,573,699,300,197,342,70,184,821
932,473,360,766,613,99,667,575,736,670,338,167,687,200,871,80,142,245,833,203
837,824,733,264,289,393,152,341,205,144,565,693,125,773,731,642,871,719,732,654
350,879,381,346,188,897,247,700,96,261,928,539,346,610,658,809,567,224,258,358";

#[allow(dead_code)]
const SAMPLES: [&str; 2] = [
    "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
    "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9",
];
