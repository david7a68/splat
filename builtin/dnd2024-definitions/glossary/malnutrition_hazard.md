---
splat_schema: 1
splat_id: 06b355bc-8fc6-593d-b2e3-6d4ac8dc9c70
splat_kind: rule
splat_key: dnd2024:glossary/malnutrition_hazard
splat_name: Malnutrition [Hazard]
splat_origin:
  key: dnd2024:reference/glossary/malnutrition_hazard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/malnutrition_hazard
  parameters:
    daily_food_pounds:
      tiny: 0.25
      small: 1
      medium: 1
      large: 4
      huge: 16
      gargantuan: 64
    partial_food:
      predicate:
        greater_than: 0
        less_than_daily_requirement_fraction: 0.5
      day_end_save:
        ability: constitution
        dc: 10
        failure_exhaustion_levels: 1
    no_food:
      days_before_first_exhaustion: 5
      day_5_end_exhaustion_levels: 1
      each_subsequent_day_end_levels: 1
      save_allowed: false
    exhaustion_cause: malnutrition
    cause_exhaustion_removal_locked_until:
      food_consumed_pounds_at_least: full_daily_requirement
  implementation: dnd2024:glossary_malnutrition_hazard
splat_contracts:
- id: dnd2024:glossary_malnutrition_hazard
  inputs:
    subject:
      type: participant
      required: true
    food_consumed_pounds:
      type: number
      required: true
    consecutive_days_without_food:
      type: integer
      required: true
  outputs:
  - exhaustion_change
  - save
  - recovery_lock
  events:
  - day.ended
  - food.consumed
  - exhaustion.removal_pending
  capabilities:
  - survival
  - hazards
  - conditions
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: a12ffc5b8cefce4d693742c3fa1ad31332777537d0118341483cadd1ee425341
  source_path: builtin/srd-5.2.1/glossary/malnutrition_hazard.md
  pages:
  - 185
  engine_gaps: &id001
  - dnd2024:glossary_malnutrition_hazard
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: aab8edd2b6c81bac941c6b92159a4ec4aad30aeef812f7cff09ac7bc0a6acf9f
splat_clauses:
- key: source_rules
  text: '# Malnutrition [Hazard] A creature needs an amount of food per day based on its size, as shown
    in the Food Needs per Day table. A creature that eats but consumes less than half the required food
    for a day must succeed on a DC 10 Constitution saving throw or gain 1 Exhaustion level at the day’s
    end. A creature that eats nothing for 5 days automatically gains 1 Exhaustion level at the end of
    the fifth day as well as an additional level at the end of each subsequent day without food. Exhaustion
    caused by malnutrition can’t be removed until the creature eats the full amount of food required for
    a day. See also “Exhaustion.” Food Needs per Day Size Food Size Food Tiny 1/4 pound Large 4 pounds
    Small 1 pound Huge 16 pounds Medium 1 pound Gargantuan 64 pounds'
  sha256: 555a418a1bea5095db43f9caf1bde2df1e1f6569ab621cc4b5a58cd23c474a17
  status: contract_specified
splat_requires_mechanics: *id001
---

# Malnutrition [Hazard]

<!-- SRD page 185 -->

A creature needs an amount of food per day based\
on its size, as shown in the Food Needs per Day ta-\
ble. A creature that eats but consumes less than half\
the required food for a day must succeed on a DC\
10 Constitution saving throw or gain 1 Exhaustion\
level at the day’s end. A creature that eats nothing\
for 5 days automatically gains 1 Exhaustion level at\
the end of the fifth day as well as an additional level\
at the end of each subsequent day without food.\
Exhaustion caused by malnutrition can’t be re-\
moved until the creature eats the full amount of\
food required for a day. See also “Exhaustion.”\
Food Needs per Day\
Size Food Size Food\
Tiny 1/4 pound Large 4 pounds\
Small 1 pound Huge 16 pounds\
Medium 1 pound Gargantuan 64 pounds\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].
