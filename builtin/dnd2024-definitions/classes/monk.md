---
splat_schema: 1
splat_id: d2158bed-a30b-5114-8de1-f38d011610c0
splat_kind: rule_group
splat_key: dnd2024:classes/monk
splat_name: Monk
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
---

# Monk

## Source text

<!-- SRD page 49 -->

Core Monk Traits\
Primary Ability Dexterity and Wisdom\
Hit Point Die D8 per Monk level\
Saving Throw Strength and Dexterity\
Proficiencies\
Skill Proficiencies Choose 2: Acrobatics, Athlet-\
ics, History, Insight, Religion,\
or Stealth\
Weapon Proficiencies Simple weapons and Martial\
weapons that have the Light\
property\
Tool Proficiencies Choose one type of Artisan’s\
Tools or Musical Instrument (see\
“Equipment”)\
Armor Training None\
Starting Equipment Choose A or B: (A) Spear, 5\
Daggers, Artisan’s Tools or\
Musical Instrument chosen\
for the tool proficiency above,\
Explorer’s Pack, and 11 GP; or\
(B) 50 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/monk ` |
| ` implementation ` | ` dnd2024:classes_monk ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` primary_abilities ` | ` ["dexterity", "wisdom"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` count_per_class_level ` | ` 1 ` |
| ` sides ` | ` 8 ` |

#### First class traits

<!-- splat:yaml path=splat_rule/parameters/first_class_traits -->

```yaml
save_proficiencies:
- strength
- dexterity
skill_proficiencies:
  count: 2
  choices:
  - acrobatics
  - athletics
  - history
  - insight
  - religion
  - stealth
weapon_proficiencies:
  simple: true
  martial_required_property_any:
  - light
armor_training: []
tool_proficiency_choice:
  count: 1
  categories:
  - artisans_tools
  - musical_instrument
```

#### Starting equipment

<!-- splat:values path=splat_rule/parameters/starting_equipment -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

##### Options

###### Equipment

<!-- splat:values path=splat_rule/parameters/starting_equipment/options/equipment -->

| Field | Value |
| --- | --- |
| ` matching_proficient_tool ` | ` true ` |

###### Items

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options/equipment/items
key: Item
order:
- spear
- dagger
- explorer_s_pack_10_gp
columns:
  Item:
    path: definition
    type: reference
  Quantity:
    path: quantity
    type: integer
-->

| Item | Quantity |
| --- | --- |
| [Spear](<../equipment/spear.md> "dnd2024:equipment/spear") | ` 1 ` |
| [Dagger](<../equipment/dagger.md> "dnd2024:equipment/dagger") | ` 5 ` |
| [Explorer’s Pack (10 GP)](<../equipment/explorer_s_pack_10_gp.md> "dnd2024:equipment/explorer_s_pack_10_gp") | ` 1 ` |

###### Currency

<!-- splat:values path=splat_rule/parameters/starting_equipment/options/equipment/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 11 ` |

###### Currency

<!-- splat:yaml path=splat_rule/parameters/starting_equipment/options/currency -->

```yaml
currency:
  gp: 50
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/monk_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- monk_level_10_heightened_focus
- monk_level_10_self_restoration
- monk_level_13_deflect_energy
- monk_level_14_disciplined_survivor
- monk_level_15_perfect_focus
- monk_level_18_superior_defense
- monk_level_19_epic_boon
- monk_level_1_martial_arts
- monk_level_1_unarmored_defense
- monk_level_20_body_and_mind
- monk_level_2_monk_s_focus
- monk_level_2_unarmored_movement
- monk_level_2_uncanny_metabolism
- monk_level_3_deflect_attacks
- monk_level_3_monk_subclass
- monk_level_4_ability_score_improvement
- monk_level_4_slow_fall
- monk_level_5_extra_attack
- monk_level_5_stunning_strike
- monk_level_6_empowered_strikes
- monk_level_7_evasion
- monk_level_9_acrobatic_movement
columns:
  Item:
    path: definition
    type: reference
  Minimum class level:
    path: minimum_class_level
    type: integer
-->

| Item | Minimum class level |
| --- | --- |
| [Level 10: Heightened Focus](<../class_features/monk_level_10_heightened_focus.md> "dnd2024:class_features/monk_level_10_heightened_focus") | ` 10 ` |
| [Level 10: Self-Restoration](<../class_features/monk_level_10_self_restoration.md> "dnd2024:class_features/monk_level_10_self_restoration") | ` 10 ` |
| [Level 13: Deflect Energy](<../class_features/monk_level_13_deflect_energy.md> "dnd2024:class_features/monk_level_13_deflect_energy") | ` 13 ` |
| [Level 14: Disciplined Survivor](<../class_features/monk_level_14_disciplined_survivor.md> "dnd2024:class_features/monk_level_14_disciplined_survivor") | ` 14 ` |
| [Level 15: Perfect Focus](<../class_features/monk_level_15_perfect_focus.md> "dnd2024:class_features/monk_level_15_perfect_focus") | ` 15 ` |
| [Level 18: Superior Defense](<../class_features/monk_level_18_superior_defense.md> "dnd2024:class_features/monk_level_18_superior_defense") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/monk_level_19_epic_boon.md> "dnd2024:class_features/monk_level_19_epic_boon") | ` 19 ` |
| [Level 1: Martial Arts](<../class_features/monk_level_1_martial_arts.md> "dnd2024:class_features/monk_level_1_martial_arts") | ` 1 ` |
| [Level 1: Unarmored Defense](<../class_features/monk_level_1_unarmored_defense.md> "dnd2024:class_features/monk_level_1_unarmored_defense") | ` 1 ` |
| [Level 20: Body and Mind](<../class_features/monk_level_20_body_and_mind.md> "dnd2024:class_features/monk_level_20_body_and_mind") | ` 20 ` |
| [Level 2: Monk’s Focus](<../class_features/monk_level_2_monk_s_focus.md> "dnd2024:class_features/monk_level_2_monk_s_focus") | ` 2 ` |
| [Level 2: Unarmored Movement](<../class_features/monk_level_2_unarmored_movement.md> "dnd2024:class_features/monk_level_2_unarmored_movement") | ` 2 ` |
| [Level 2: Uncanny Metabolism](<../class_features/monk_level_2_uncanny_metabolism.md> "dnd2024:class_features/monk_level_2_uncanny_metabolism") | ` 2 ` |
| [Level 3: Deflect Attacks](<../class_features/monk_level_3_deflect_attacks.md> "dnd2024:class_features/monk_level_3_deflect_attacks") | ` 3 ` |
| [Level 3: Monk Subclass](<../class_features/monk_level_3_monk_subclass.md> "dnd2024:class_features/monk_level_3_monk_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/monk_level_4_ability_score_improvement.md> "dnd2024:class_features/monk_level_4_ability_score_improvement") | ` 4 ` |
| [Level 4: Slow Fall](<../class_features/monk_level_4_slow_fall.md> "dnd2024:class_features/monk_level_4_slow_fall") | ` 4 ` |
| [Level 5: Extra Attack](<../class_features/monk_level_5_extra_attack.md> "dnd2024:class_features/monk_level_5_extra_attack") | ` 5 ` |
| [Level 5: Stunning Strike](<../class_features/monk_level_5_stunning_strike.md> "dnd2024:class_features/monk_level_5_stunning_strike") | ` 5 ` |
| [Level 6: Empowered Strikes](<../class_features/monk_level_6_empowered_strikes.md> "dnd2024:class_features/monk_level_6_empowered_strikes") | ` 6 ` |
| [Level 7: Evasion](<../class_features/monk_level_7_evasion.md> "dnd2024:class_features/monk_level_7_evasion") | ` 7 ` |
| [Level 9: Acrobatic Movement](<../class_features/monk_level_9_acrobatic_movement.md> "dnd2024:class_features/monk_level_9_acrobatic_movement") | ` 9 ` |
