use std::cmp;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    start: u64,
    end: u64,
    size: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range {
            start,
            end,
            size: end - start,
        }
    }

    fn _split(self, index: u64) -> (Range, Range) {
        (
            Range {
                start: self.start,
                end: index,
                size: index - self.start,
            },
            Range {
                start: index + 1,
                end: self.end,
                size: self.end - (index + 1),
            },
        )
    }
}

impl Iterator for Range {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.start == self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Map {
    from: u64,
    to: u64,
    length: u64,
}

impl Map {
    fn convert(self, number: u64) -> Option<u64> {
        if number < self.from || self.from + self.length <= number {
            return None;
        }

        let delta = number - self.from;
        Some(self.to + delta)
    }
}

#[derive(Debug, Clone)]
pub struct Mapper {
    _name: String,
    maps: Vec<Map>,
}

impl Mapper {
    fn new(name: &str) -> Mapper {
        Mapper {
            _name: name.to_string(),
            maps: vec![],
        }
    }

    fn add_map(&mut self, line: &str) -> &mut Self {
        let mut pieces = line.split(" ").map(|n| n.parse::<u64>().unwrap());
        let map = Map {
            to: pieces.next().unwrap_or(0),
            from: pieces.next().unwrap_or(0),
            length: pieces.next().unwrap_or(0),
        };
        self.maps.push(map);
        self
    }

    fn convert(&self, seed: u64) -> u64 {
        let result: Vec<u64> = self
            .maps
            .iter()
            .filter_map(|map| map.convert(seed))
            .collect();
        if result.len() == 0 {
            // println!("{} seed {} not converted", self._name, seed);
            return seed;
        } else if result.len() > 1 {
            panic!(
                "Weird, this should only have one matcher: {}, {:?}",
                seed, self
            );
        }
        // println!("{} seed {} converted to {}", self._name, seed, result[0]);
        result[0]
    }
}

pub fn build_mappers(mut iter: std::slice::Iter<'_, String>) -> Vec<Mapper> {
    let mut mappers: Vec<Option<Mapper>> = vec![];
    let mut mapper: Option<Mapper> = None;
    loop {
        if let Some(line) = iter.next() {
            if line == "" {
                // println!("mapper {:?}", mapper);
                mappers.push(mapper);
                mapper = None;
                continue;
            }

            if let Some(ref mut m) = mapper {
                m.add_map(line);
            } else {
                mapper = Some(Mapper::new(line))
            }
        } else {
            mappers.push(mapper);
            break;
        }
    }

    mappers.into_iter().flatten().collect()
}

pub fn get_location(seed: u64, mappers: &Vec<Mapper>) -> u64 {
    mappers.iter().fold(seed, |acc, val| val.convert(acc))
}

pub fn get_locations(seeds: Vec<u64>, mappers: Vec<Mapper>) -> Vec<u64> {
    seeds
        .iter()
        .map(|seed| get_location(*seed, &mappers))
        .collect()
}

pub fn smallest(locations: Vec<u64>) -> u64 {
    locations
        .iter()
        .fold(locations[0], |acc, val| cmp::min(acc, *val))
}

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut iter = input.iter();
    let seeds: Vec<_> = iter
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap_or(0))
        .collect();

    // println!("Seeds {:?}", seeds);
    iter.next();
    let mappers = build_mappers(iter);

    let locations = get_locations(seeds, mappers);
    // println!("locations {:?}", locations);
    let smallest = smallest(locations);

    // println!("smallest location {}", smallest);
    smallest
}

pub fn build_ranges(seed_ranges: Vec<u64>) -> Vec<Range> {
    let mut ranges: Vec<Range> = vec![];
    for i in (0..seed_ranges.len()).step_by(2) {
        ranges.push(Range::new(
            seed_ranges[i],
            seed_ranges[i] + seed_ranges[i + 1],
        ));
    }
    ranges
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let mut iter = input.iter();
    let seed_ranges: Vec<_> = iter
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap_or(0))
        .collect();

    let ranges: Vec<Range> = build_ranges(seed_ranges);
    println!("Ranges {:?}", ranges);
    let mappers = build_mappers(iter);

    let mut smallest_location: u64 = u64::MAX;
    let total = ranges.iter().fold(0, |acc, val| acc + val.size);
    let now = Instant::now();
    println!("Starting computation of {} seeds", total);
    let mut count: u64 = 0;

    let (tx, rx) = mpsc::channel();
    for range in ranges {
        let tx = tx.clone();
        let mappers = mappers.clone();

        thread::spawn(move || {
            for seed in range {
                let location = get_location(seed, &mappers);
                tx.send(location).unwrap();
            }
            drop(tx)
        });
    }
    drop(tx);

    for location in rx {
        if count % 100000 == 0 {
            println!(
                "Elapsed time: {:?}, count: {}/{}, percentage: {:05.2}%, smallest_location: {}",
                now.elapsed(),
                count,
                total,
                count / total,
                smallest_location
            );
        }
        smallest_location = cmp::min(smallest_location, location);
        count += 1;
    }

    smallest_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        52 50 48
        50 98 2
        
        soil-to-fertilizer map:
        37 52 2
        39 0 15
        0 15 37
        
        fertilizer-to-water map:
        0 11 42
        49 53 8
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 35);
    }

    #[test]
    fn convert_test() {
        let input = "seed-to-soil map:
        50 98 2
        52 50 48";
        let mut lines = input.lines().map(|l| l.trim());

        let mut mapper = Mapper::new(lines.next().unwrap());
        mapper.add_map(lines.next().unwrap());
        mapper.add_map(lines.next().unwrap());

        assert_eq!(mapper.clone().convert(98), 50);
        assert_eq!(mapper.clone().convert(99), 51);
        assert_eq!(mapper.clone().convert(100), 100);
        assert_eq!(mapper.clone().convert(49), 49);
        assert_eq!(mapper.clone().convert(50), 52);
    }

    #[test]
    fn part_1_test2() {
        let input = "seeds: 222541566 218404460 670428364 432472902 2728902838 12147727 3962570697 52031641 2849288350 113747257 3648852659 73423293 4036058422 190602154 1931540843 584314999 3344622241 180428346 1301166628 310966761

        seed-to-soil map:
        357888202 777841571 45089383
        1091769591 2222785614 212172358
        747211456 668867483 108974088
        635547523 229171508 111663933
        1573402842 2067459960 102412403
        4289698662 1148443045 5268634
        2395735068 1322252554 161006801
        0 205577967 4476015
        57955189 635249899 15743396
        402977585 0 26133741
        464695488 412326242 24494890
        2966156314 1301354919 3630744
        42819069 472405294 15136120
        3261136238 2193516168 29269446
        2394617262 1304985663 1117806
        3573311427 1725292988 265415981
        3184385247 1990708969 76750991
        36407558 487541414 6411511
        178443976 26133741 179444226
        4233901482 2444128986 55797180
        2556741869 2169872363 23643805
        429111326 436821132 35584162
        4476015 622435882 12814017
        2830010004 4158820986 136146310
        1315195976 890236179 258206866
        17290032 210053982 19117526
        2969787058 3404926809 66954949
        106953175 340835441 71490801
        3307388168 3846327739 265923259
        2580385674 3488864242 249624330
        3036742007 1153711679 147643240
        3290405684 3471881758 16982484
        4187331494 4112250998 46569988
        2378468177 1306103469 16149085
        1308440551 2936378300 6755425
        73698585 822930954 33254590
        489190378 493952925 128482957
        1675815245 1483259355 242033633
        3849072207 2598119013 338259287
        1917848878 2944307510 460619299
        1303941949 885737577 4498602
        3847898422 2943133725 1173785
        617673335 650993295 17874188
        993576744 2499926166 98192847
        3838727408 2434957972 9171014
        885737577 3738488572 107839167
        
        soil-to-fertilizer map:
        146677673 622659563 93167301
        1157991666 1089969349 162939558
        3393833863 2765693883 993201
        1089969349 2528480087 47120899
        1536155605 1455175707 97002831
        77735382 829301434 68942291
        519819619 496909514 125750049
        1633158436 2638516375 127177508
        2914682387 1777568650 8769904
        1907493268 2936307342 209538906
        1523198024 2370144872 12957581
        398047045 375136940 121772574
        3155987938 1552178538 225390112
        2722437809 1786338554 7281125
        2405956502 1793619679 316481307
        4079025823 3374787635 121920527
        1020706608 898243725 11125486
        1320931224 1252908907 202266800
        4200946350 4161901201 91091718
        2872708010 4252992919 23854892
        2896562902 4276847811 18119485
        2923452291 2766687084 169620258
        1760335944 2261396563 6024603
        362307857 715826864 35739188
        4292038068 2525550859 2929228
        2345973561 2310161931 59982941
        3426288597 3496708162 124839972
        2768316300 2383102453 51488089
        2729718934 2455491960 38597366
        1137090248 2434590542 20901418
        2819804389 2110100986 52903621
        3394827064 2494089326 31461533
        1809101312 2163004607 98391956
        3551128569 3621548134 527897254
        1766360547 2267421166 42740765
        2117032174 3145846248 228941387
        645569668 0 375136940
        0 751566052 77735382
        239844974 925431556 122462883
        3093072549 2575600986 62915389
        1031832094 909369211 16062345
        3381378050 4149445388 12455813
        
        fertilizer-to-water map:
        1853863567 1354319094 57762399
        1963297596 2892133710 74523026
        351355449 2825239457 1556531
        4263216859 3376413885 31750437
        2555584073 3055976839 118124436
        2730998967 1343408443 10910651
        1638376751 979599391 153825342
        978844708 811605824 82508702
        1911625966 894114526 6715081
        3367668240 3408164322 203176531
        2729654714 3174101275 1344253
        3200258520 795809557 15796267
        1595497633 1782591008 42879118
        2683440491 1412081493 46214223
        1958950306 101242881 4347290
        2831229721 242278649 124658639
        1792202093 734148083 61661474
        2741909618 2966656736 89320103
        1918341047 3175445528 40609259
        2955888360 1825470126 7069388
        3594735667 3635231749 181844678
        2673708509 412658344 9731982
        2962957748 1832539514 237300772
        530398473 105590171 136688478
        1177133692 366937288 45721056
        27883870 2489305043 153451249
        352911980 2711533741 113705716
        3570844771 3611340853 23890896
        466617696 900829607 30422191
        0 2461421173 27883870
        3776580345 3360472089 15941796
        4207783288 3817076427 55433571
        2429401509 1249205015 60844842
        181335119 2642756292 68777449
        1061353410 1133424733 115780282
        667086951 422390326 311757757
        2490246351 2826795988 65337722
        1222854748 931251798 48347593
        1271202341 1458295716 324295292
        3360472089 4287771145 7196151
        497039887 1310049857 33358586
        2037820622 2069840286 391580887
        250112568 0 101242881
        3792522141 3872509998 415261147
        
        water-to-light map:
        3392354816 2147745556 27501466
        1878871951 1725889381 75406426
        173441126 161378219 150090654
        1018905669 635099142 165518615
        4087062942 1662057640 30823678
        353004829 1119237890 172394750
        1845863888 1692881318 33008063
        153206340 867143452 20234786
        2604702405 4037570714 169107986
        1617110925 3533554525 169325605
        2559755690 1617110925 44946715
        600097153 912061633 124651296
        3946007322 3319554557 141055620
        2465332301 3460610177 72944348
        724748449 340941922 294157220
        3044149982 2175247022 348204834
        2773810391 2633005641 270339591
        525399579 153206340 8171879
        2139921020 3702880130 325411281
        3419856282 1978376483 109941715
        1954278377 2523451856 88074744
        2538276649 2611526600 21479041
        4117886620 1801295807 177080676
        2051632424 4206678700 88288596
        2042353121 4028291411 9279303
        3529797997 2903345232 416209325
        1786436530 2088318198 59427358
        533571458 800617757 66525695
        1184424284 1036712929 82524961
        1266949245 887378238 24683395
        323531780 311468873 29473049
        
        light-to-temperature map:
        2119656026 2694441768 51718564
        582739623 515860164 331142829
        3188466529 3433161801 686443431
        1208858214 1852905471 223488011
        2984018793 2746160332 20792175
        114072893 923500840 335770555
        1078092281 1568663312 55313164
        1528379045 847002993 76497847
        3159539374 3404234646 28927155
        1604876892 44343574 471516590
        3874909960 2119656026 229635679
        913882452 1404453483 164209829
        449843448 1720009296 132896175
        3004810968 2539713362 154728406
        1432346225 1623976476 96032820
        2171374590 4119605232 175362064
        2827425140 3247640993 156593653
        1177749019 1259271395 31109195
        2346736654 2766952507 480688486
        1133405445 0 44343574
        0 1290380590 114072893
        4104545639 2349291705 190421657
        
        temperature-to-humidity map:
        4032879828 4294798436 168860
        4033048688 927598400 261918608
        0 101834652 315424112
        3415479218 1417189709 517560635
        3933039853 4175022329 99839975
        2470365900 2513768139 498846371
        2059440409 4274862304 19936132
        2969212271 2295173893 218594246
        315424112 417258764 36139086
        351563198 0 101834652
        927598400 1934750344 360423549
        3187806517 1189517008 66595582
        3254402099 1256112590 161077119
        2079376541 3784032970 390989359
        1288021949 3012614510 771418460
        
        humidity-to-location map:
        1627636687 2047220773 460084702
        2744823277 560934787 184170906
        3485120052 3437939311 342343442
        3959782982 3213426504 221968093
        1599327759 745105693 28308928
        2311378749 773414621 172869650
        4292422582 3435394597 2544714
        4181751075 3780282753 110671507
        2484248399 1371539463 260574878
        944796451 1771589193 23241552
        2985880462 4036578393 258388903
        896379083 2507305475 48417368
        2298631675 0 12747074
        237282755 12747074 435619394
        812377001 1310308951 61230512
        1486759440 448366468 112568319
        3827463494 2985880462 132319488
        3389893498 3118199950 95226554
        0 1073026196 237282755
        672902149 1632114341 139474852
        3244269365 3890954260 145624133
        1103441152 2555722843 279347933
        2181644796 1794830745 116986879
        873607513 1050254626 22771570
        968038003 1911817624 135403149
        2087721389 2835070776 93923407
        1382789085 946284271 103970355";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_1(&lines), 379811651)
    }

    #[test]
    fn build_ranges_test() {
        let input: Vec<u64> = vec![0, 5, 10, 10, 20, 20];

        assert_eq!(
            build_ranges(input),
            vec![
                Range {
                    start: 0,
                    end: 5,
                    size: 5,
                },
                Range {
                    start: 10,
                    end: 20,
                    size: 10,
                },
                Range {
                    start: 20,
                    end: 40,
                    size: 20
                }
            ]
        );
    }

    #[test]
    fn part_2_test() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        52 50 48
        50 98 2
        
        soil-to-fertilizer map:
        37 52 2
        39 0 15
        0 15 37
        
        fertilizer-to-water map:
        0 11 42
        49 53 8
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let lines: Vec<String> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_2(&lines), 46);
    }
}
