---
splat_schema: 2
splat_id: 507bda38-bdc0-529f-83be-3632e295489e
splat_kind: rule_group
splat_key: dnd2024:classes/rogue
splat_name: Rogue
splat_origin: dnd2024:reference/pages/061
splat_sources:
- SRD 5.2.1, pp. 61, https://www.dndbeyond.com/srd
---

# Rogue

## Source text

<!-- SRD page 61 -->

**Core Rogue Traits**

|  |  |
| --- | --- |
| Primary Ability | Dexterity |
| Hit Point Die | D8 per Rogue level |
| Saving Throw | Dexterity and Intelligence |
| Proficiencies |  |
| Skill Proficiencies | Choose 4: Acrobatics, Athletics, Deception, Insight, Intimidation, Investigation, Perception, Persuasion, Sleight of Hand, or Stealth |
| Weapon Proficiencies | Simple weapons and Martial weapons that have the Finesse or Light property |
| Tool Proficiencies | Thieves’ Tools |
| Armor Training | Light armor |
| Starting Equipment | Choose A or B: (A) Leather Armor, 2 Daggers, Shortsword, Shortbow, 20 Arrows, Quiver, Thieves’ Tools, Burglar’s Pack, and 8 GP; or (B) 100 GP |

**Rogue Features**

| Level | Proficiency Bonus | Class Features | Sneak Attack |
| --- | --- | --- | --- |
| 1 | +2 | Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery | 1d6 |
| 2 | +2 | Cunning Action | 1d6 |
| 3 | +2 | Rogue Subclass, Steady Aim | 2d6 |
| 4 | +2 | Ability Score Improvement | 2d6 |
| 5 | +3 | Cunning Strike, Uncanny Dodge | 3d6 |
| 6 | +3 | Expertise | 3d6 |
| 7 | +3 | Evasion, Reliable Talent | 4d6 |
| 8 | +3 | Ability Score Improvement | 4d6 |
| 9 | +4 | Subclass feature | 5d6 |
| 10 | +4 | Ability Score Improvement | 5d6 |
| 11 | +4 | Improved Cunning Strike | 6d6 |
| 12 | +4 | Ability Score Improvement | 6d6 |
| 13 | +5 | Subclass feature | 7d6 |
| 14 | +5 | Devious Strikes | 7d6 |
| 15 | +5 | Slippery Mind | 8d6 |
| 16 | +5 | Ability Score Improvement | 8d6 |
| 17 | +6 | Subclass feature | 9d6 |
| 18 | +6 | Elusive | 9d6 |
| 19 | +6 | Epic Boon | 10d6 |
| 20 | +6 | Stroke of Luck | 10d6 |

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/rogue ` |
| ` implementation ` | ` dnd2024:classes_rogue ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` primary_abilities ` | ` ["dexterity"] ` |

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
- dexterity
- intelligence
skill_proficiencies:
  count: 4
  choices:
  - acrobatics
  - athletics
  - deception
  - insight
  - intimidation
  - investigation
  - perception
  - persuasion
  - sleight_of_hand
  - stealth
weapon_proficiencies:
  simple: true
  martial_required_property_any:
  - finesse
  - light
armor_training:
- light
tool_proficiencies:
- key: dnd2024:equipment/other_tools_thieves_tools_25_gp
```

#### Starting equipment

<!-- splat:values path=splat_rule/parameters/starting_equipment -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

##### Options

###### Equipment

###### Items

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options/equipment/items
key: Item
order:
- leather_armor
- dagger
- shortsword
- shortbow
- ammunition_varies
- quiver_1_gp
- other_tools_thieves_tools_25_gp
- burglar_s_pack_16_gp
columns:
  Item:
    path: definition
    type: reference
  Quantity:
    path: quantity
    type: integer
  Variant:
    path: variant
    type: symbol
    optional: true
-->

| Item | Quantity | Variant |
| --- | --- | --- |
| [Leather Armor](<../equipment/leather_armor.md> "dnd2024:equipment/leather_armor") | ` 1 ` |  |
| [Dagger](<../equipment/dagger.md> "dnd2024:equipment/dagger") | ` 2 ` |  |
| [Shortsword](<../equipment/shortsword.md> "dnd2024:equipment/shortsword") | ` 1 ` |  |
| [Shortbow](<../equipment/shortbow.md> "dnd2024:equipment/shortbow") | ` 1 ` |  |
| [Ammunition (Varies)](<../equipment/ammunition_varies.md> "dnd2024:equipment/ammunition_varies") | ` 20 ` | ` Arrows ` |
| [Quiver (1 GP)](<../equipment/quiver_1_gp.md> "dnd2024:equipment/quiver_1_gp") | ` 1 ` |  |
| [Thieves’ Tools (25 GP)](<../equipment/other_tools_thieves_tools_25_gp.md> "dnd2024:equipment/other_tools_thieves_tools_25_gp") | ` 1 ` |  |
| [Burglar’s Pack (16 GP)](<../equipment/burglar_s_pack_16_gp.md> "dnd2024:equipment/burglar_s_pack_16_gp") | ` 1 ` |  |

###### Currency

<!-- splat:values path=splat_rule/parameters/starting_equipment/options/equipment/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 8 ` |

###### Currency

<!-- splat:yaml path=splat_rule/parameters/starting_equipment/options/currency -->

```yaml
currency:
  gp: 100
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/rogue_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- rogue_level_11_improved_cunning_strike
- rogue_level_14_devious_strikes
- rogue_level_15_slippery_mind
- rogue_level_18_elusive
- rogue_level_19_epic_boon
- rogue_level_1_expertise
- rogue_level_1_sneak_attack
- rogue_level_1_thieves_cant
- rogue_level_1_weapon_mastery
- rogue_level_20_stroke_of_luck
- rogue_level_2_cunning_action
- rogue_level_3_rogue_subclass
- rogue_level_3_steady_aim
- rogue_level_4_ability_score_improvement
- rogue_level_5_cunning_strike
- rogue_level_5_uncanny_dodge
- rogue_level_7_evasion
- rogue_level_7_reliable_talent
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
| [Level 11: Improved Cunning Strike](<../class_features/rogue_level_11_improved_cunning_strike.md> "dnd2024:class_features/rogue_level_11_improved_cunning_strike") | ` 11 ` |
| [Level 14: Devious Strikes](<../class_features/rogue_level_14_devious_strikes.md> "dnd2024:class_features/rogue_level_14_devious_strikes") | ` 14 ` |
| [Level 15: Slippery Mind](<../class_features/rogue_level_15_slippery_mind.md> "dnd2024:class_features/rogue_level_15_slippery_mind") | ` 15 ` |
| [Level 18: Elusive](<../class_features/rogue_level_18_elusive.md> "dnd2024:class_features/rogue_level_18_elusive") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/rogue_level_19_epic_boon.md> "dnd2024:class_features/rogue_level_19_epic_boon") | ` 19 ` |
| [Level 1: Expertise](<../class_features/rogue_level_1_expertise.md> "dnd2024:class_features/rogue_level_1_expertise") | ` 1 ` |
| [Level 1: Sneak Attack](<../class_features/rogue_level_1_sneak_attack.md> "dnd2024:class_features/rogue_level_1_sneak_attack") | ` 1 ` |
| [Level 1: Thieves’ Cant](<../class_features/rogue_level_1_thieves_cant.md> "dnd2024:class_features/rogue_level_1_thieves_cant") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/rogue_level_1_weapon_mastery.md> "dnd2024:class_features/rogue_level_1_weapon_mastery") | ` 1 ` |
| [Level 20: Stroke of Luck](<../class_features/rogue_level_20_stroke_of_luck.md> "dnd2024:class_features/rogue_level_20_stroke_of_luck") | ` 20 ` |
| [Level 2: Cunning Action](<../class_features/rogue_level_2_cunning_action.md> "dnd2024:class_features/rogue_level_2_cunning_action") | ` 2 ` |
| [Level 3: Rogue Subclass](<../class_features/rogue_level_3_rogue_subclass.md> "dnd2024:class_features/rogue_level_3_rogue_subclass") | ` 3 ` |
| [Level 3: Steady Aim](<../class_features/rogue_level_3_steady_aim.md> "dnd2024:class_features/rogue_level_3_steady_aim") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/rogue_level_4_ability_score_improvement.md> "dnd2024:class_features/rogue_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Cunning Strike](<../class_features/rogue_level_5_cunning_strike.md> "dnd2024:class_features/rogue_level_5_cunning_strike") | ` 5 ` |
| [Level 5: Uncanny Dodge](<../class_features/rogue_level_5_uncanny_dodge.md> "dnd2024:class_features/rogue_level_5_uncanny_dodge") | ` 5 ` |
| [Level 7: Evasion](<../class_features/rogue_level_7_evasion.md> "dnd2024:class_features/rogue_level_7_evasion") | ` 7 ` |
| [Level 7: Reliable Talent](<../class_features/rogue_level_7_reliable_talent.md> "dnd2024:class_features/rogue_level_7_reliable_talent") | ` 7 ` |
