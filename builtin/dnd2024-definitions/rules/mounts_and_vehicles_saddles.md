---
splat_schema: 1
splat_id: 5ed175ce-1faa-5182-8c02-eab4dcf4638d
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_saddles
splat_name: Saddles
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
---

# Saddles

## Source text

<!-- SRD page 100 -->

A saddle comes with a bit, a bridle, reins, and any\
other equipment needed to use the saddle. A Mil-\
itary Saddle gives Advantage on any ability check\
you make to remain mounted. An Exotic Saddle is\
required for riding an aquatic or a flying mount.\
Mounts and Other Animals\
Item Carrying Capacity Cost\
Camel 450 lb. 50 GP\
Elephant 1,320 lb. 200 GP\
Horse, Draft 540 lb. 50 GP\
Horse, Riding 480 lb. 75 GP\
Mastiff 195 lb. 25 GP\
Mule 420 lb. 8 GP\
Pony 225 lb. 30 GP\
Warhorse 540 lb. 400 GP\
Tack, Harness, and Drawn Vehicles\
Item Weight Cost\
Carriage 600 lb. 100 GP\
Cart 200 lb. 15 GP\
Chariot 100 lb. 250 GP\
Feed per day 10 lb. 5 CP\
Saddle\
Exotic 40 lb. 60 GP\
Military 30 lb. 20 GP\
Riding 25 lb. 10 GP\
Sled 300 lb. 20 GP\
Stabling per day — 5 SP\
Wagon 400 lb. 35 GP\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/mounts_and_vehicles_saddles ` |
| ` implementation ` | ` dnd2024:rules_mounts_and_vehicles_saddles ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` saddle_includes ` | ` ["bit", "bridle", "reins", "other_required_equipment"] ` |
| ` exotic_saddle_required_for ` | ` ["aquatic_mount", "flying_mount"] ` |

#### Military saddle

<!-- splat:values path=splat_rule/parameters/military_saddle -->

| Field | Value |
| --- | --- |
| ` remain_mounted_ability_checks ` | ` advantage ` |

#### Mounts

<!-- splat:rows path=splat_rule/parameters/mounts -->

| Row | kind | capacity_lb | cost_gp |
| --- | --- | --- | --- |
| ` 0 ` | ` camel ` | ` 450 ` | ` 50 ` |
| ` 1 ` | ` elephant ` | ` 1320 ` | ` 200 ` |
| ` 2 ` | ` draft_horse ` | ` 540 ` | ` 50 ` |
| ` 3 ` | ` riding_horse ` | ` 480 ` | ` 75 ` |
| ` 4 ` | ` mastiff ` | ` 195 ` | ` 25 ` |
| ` 5 ` | ` mule ` | ` 420 ` | ` 8 ` |
| ` 6 ` | ` pony ` | ` 225 ` | ` 30 ` |
| ` 7 ` | ` warhorse ` | ` 540 ` | ` 400 ` |

#### Tack and vehicles

<!-- splat:rows path=splat_rule/parameters/tack_and_vehicles -->

| Row | kind | weight_lb | cost_gp |
| --- | --- | --- | --- |
| ` 0 ` | ` carriage ` | ` 600 ` | ` 100 ` |
| ` 1 ` | ` cart ` | ` 200 ` | ` 15 ` |
| ` 2 ` | ` chariot ` | ` 100 ` | ` 250 ` |
| ` 3 ` | ` feed_per_day ` | ` 10 ` | ` 0.05 ` |
| ` 4 ` | ` exotic_saddle ` | ` 40 ` | ` 60 ` |
| ` 5 ` | ` military_saddle ` | ` 30 ` | ` 20 ` |
| ` 6 ` | ` riding_saddle ` | ` 25 ` | ` 10 ` |
| ` 7 ` | ` sled ` | ` 300 ` | ` 20 ` |
| ` 8 ` | ` stabling_per_day ` | ` null ` | ` 0.5 ` |
| ` 9 ` | ` wagon ` | ` 400 ` | ` 35 ` |
