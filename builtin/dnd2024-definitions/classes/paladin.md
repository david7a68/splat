---
splat_schema: 2
splat_id: f4823d01-8fa3-5e60-9f72-1af3248f7d9f
splat_kind: rule_group
splat_key: dnd2024:classes/paladin
splat_name: Paladin
splat_origin: dnd2024:reference/pages/053
splat_sources:
- SRD 5.2.1, pp. 53, https://www.dndbeyond.com/srd
---

# Paladin

## Source text

<!-- SRD page 53 -->

**Core Paladin Traits**

|  |  |
| --- | --- |
| Primary Ability | Strength and Charisma |
| Hit Point Die | D10 per Paladin level |
| Saving Throw | Wisdom and Charisma |
| Proficiencies |  |
| Skill Proficiencies | Choose 2: Athletics, Insight, Intimidation, Medicine, Persuasion, or Religion |
| Weapon Proficiencies | Simple and Martial weapons |
| Armor Training | Light, Medium, and Heavy armor and Shields |
| Starting Equipment | Choose A or B: (A) Chain Mail, Shield, Longsword, 6 Javelins, Holy Symbol, Priest’s Pack, and 9 GP; or (B) 150 GP |

**Paladin Features**

| Level | Proficiency Bonus | Class Features | Channel Divinity | Prepared Spells | Level 1 Slots | Level 2 Slots | Level 3 Slots | Level 4 Slots | Level 5 Slots |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | +2 | Lay On Hands, Spellcasting, Weapon Mastery | — | 2 | 2 | — | — | — | — |
| 2 | +2 | Fighting Style, Paladin’s Smite | — | 3 | 2 | — | — | — | — |
| 3 | +2 | Channel Divinity, Paladin Subclass | 2 | 4 | 3 | — | — | — | — |
| 4 | +2 | Ability Score Improvement | 2 | 5 | 3 | — | — | — | — |
| 5 | +3 | Extra Attack, Faithful Steed | 2 | 6 | 4 | 2 | — | — | — |
| 6 | +3 | Aura of Protection | 2 | 6 | 4 | 2 | — | — | — |
| 7 | +3 | Subclass feature | 2 | 7 | 4 | 3 | — | — | — |
| 8 | +3 | Ability Score Improvement | 2 | 7 | 4 | 3 | — | — | — |
| 9 | +4 | Abjure Foes | 2 | 9 | 4 | 3 | 2 | — | — |
| 10 | +4 | Aura of Courage | 2 | 9 | 4 | 3 | 2 | — | — |
| 11 | +4 | Radiant Strikes | 3 | 10 | 4 | 3 | 3 | — | — |
| 12 | +4 | Ability Score Improvement | 3 | 10 | 4 | 3 | 3 | — | — |
| 13 | +5 | — | 3 | 11 | 4 | 3 | 3 | 1 | — |
| 14 | +5 | Restoring Touch | 3 | 11 | 4 | 3 | 3 | 1 | — |
| 15 | +5 | Subclass feature | 3 | 12 | 4 | 3 | 3 | 2 | — |
| 16 | +5 | Ability Score Improvement | 3 | 12 | 4 | 3 | 3 | 2 | — |
| 17 | +6 | — | 3 | 14 | 4 | 3 | 3 | 3 | 1 |
| 18 | +6 | Aura Expansion | 3 | 14 | 4 | 3 | 3 | 3 | 1 |
| 19 | +6 | Epic Boon | 3 | 15 | 4 | 3 | 3 | 3 | 2 |
| 20 | +6 | Subclass feature | 3 | 15 | 4 | 3 | 3 | 3 | 2 |

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/paladin ` |
| ` implementation ` | ` dnd2024:classes_paladin ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` primary_abilities ` | ` ["strength", "charisma"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` count_per_class_level ` | ` 1 ` |
| ` sides ` | ` 10 ` |

#### First class traits

<!-- splat:yaml path=splat_rule/parameters/first_class_traits -->

```yaml
save_proficiencies:
- wisdom
- charisma
skill_proficiencies:
  count: 2
  choices:
  - athletics
  - insight
  - intimidation
  - medicine
  - persuasion
  - religion
weapon_proficiencies:
  simple: true
  martial: true
armor_training:
- light
- medium
- heavy
- shield
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
- chain_mail
- shield
- longsword
- javelin
- holy_symbol_varies
- priest_s_pack_33_gp
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
| [Chain Mail](<../equipment/chain_mail.md> "dnd2024:equipment/chain_mail") | ` 1 ` |
| [Shield](<../equipment/shield.md> "dnd2024:equipment/shield") | ` 1 ` |
| [Longsword](<../equipment/longsword.md> "dnd2024:equipment/longsword") | ` 1 ` |
| [Javelin](<../equipment/javelin.md> "dnd2024:equipment/javelin") | ` 6 ` |
| [Holy Symbol (Varies)](<../equipment/holy_symbol_varies.md> "dnd2024:equipment/holy_symbol_varies") | ` 1 ` |
| [Priest’s Pack (33 GP)](<../equipment/priest_s_pack_33_gp.md> "dnd2024:equipment/priest_s_pack_33_gp") | ` 1 ` |

###### Currency

<!-- splat:values path=splat_rule/parameters/starting_equipment/options/equipment/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 9 ` |

###### Currency

<!-- splat:yaml path=splat_rule/parameters/starting_equipment/options/currency -->

```yaml
currency:
  gp: 150
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/paladin_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- paladin_level_10_aura_of_courage
- paladin_level_11_radiant_strikes
- paladin_level_14_restoring_touch
- paladin_level_18_aura_expansion
- paladin_level_19_epic_boon
- paladin_level_1_lay_on_hands
- paladin_level_1_spellcasting
- paladin_level_1_weapon_mastery
- paladin_level_2_fighting_style
- paladin_level_2_paladin_s_smite
- paladin_level_3_channel_divinity
- paladin_level_3_paladin_subclass
- paladin_level_4_ability_score_improvement
- paladin_level_5_extra_attack
- paladin_level_5_faithful_steed
- paladin_level_6_aura_of_protection
- paladin_level_9_abjure_foes
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
| [Level 10: Aura of Courage](<../class_features/paladin_level_10_aura_of_courage.md> "dnd2024:class_features/paladin_level_10_aura_of_courage") | ` 10 ` |
| [Level 11: Radiant Strikes](<../class_features/paladin_level_11_radiant_strikes.md> "dnd2024:class_features/paladin_level_11_radiant_strikes") | ` 11 ` |
| [Level 14: Restoring Touch](<../class_features/paladin_level_14_restoring_touch.md> "dnd2024:class_features/paladin_level_14_restoring_touch") | ` 14 ` |
| [Level 18: Aura Expansion](<../class_features/paladin_level_18_aura_expansion.md> "dnd2024:class_features/paladin_level_18_aura_expansion") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/paladin_level_19_epic_boon.md> "dnd2024:class_features/paladin_level_19_epic_boon") | ` 19 ` |
| [Level 1: Lay On Hands](<../class_features/paladin_level_1_lay_on_hands.md> "dnd2024:class_features/paladin_level_1_lay_on_hands") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/paladin_level_1_spellcasting.md> "dnd2024:class_features/paladin_level_1_spellcasting") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/paladin_level_1_weapon_mastery.md> "dnd2024:class_features/paladin_level_1_weapon_mastery") | ` 1 ` |
| [Level 2: Fighting Style](<../class_features/paladin_level_2_fighting_style.md> "dnd2024:class_features/paladin_level_2_fighting_style") | ` 2 ` |
| [Level 2: Paladin’s Smite](<../class_features/paladin_level_2_paladin_s_smite.md> "dnd2024:class_features/paladin_level_2_paladin_s_smite") | ` 2 ` |
| [Level 3: Channel Divinity](<../class_features/paladin_level_3_channel_divinity.md> "dnd2024:class_features/paladin_level_3_channel_divinity") | ` 3 ` |
| [Level 3: Paladin Subclass](<../class_features/paladin_level_3_paladin_subclass.md> "dnd2024:class_features/paladin_level_3_paladin_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/paladin_level_4_ability_score_improvement.md> "dnd2024:class_features/paladin_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Extra Attack](<../class_features/paladin_level_5_extra_attack.md> "dnd2024:class_features/paladin_level_5_extra_attack") | ` 5 ` |
| [Level 5: Faithful Steed](<../class_features/paladin_level_5_faithful_steed.md> "dnd2024:class_features/paladin_level_5_faithful_steed") | ` 5 ` |
| [Level 6: Aura of Protection](<../class_features/paladin_level_6_aura_of_protection.md> "dnd2024:class_features/paladin_level_6_aura_of_protection") | ` 6 ` |
| [Level 9: Abjure Foes](<../class_features/paladin_level_9_abjure_foes.md> "dnd2024:class_features/paladin_level_9_abjure_foes") | ` 9 ` |
