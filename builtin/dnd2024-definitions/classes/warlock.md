---
splat_schema: 2
splat_id: b2637fe0-2148-5654-b2c3-916cac461834
splat_kind: rule_group
splat_key: dnd2024:classes/warlock
splat_name: Warlock
splat_origin: dnd2024:reference/pages/070
splat_sources:
- SRD 5.2.1, pp. 70, https://www.dndbeyond.com/srd
---

# Warlock

## Source text

<!-- SRD page 70 -->

**Core Warlock Traits**

|  |  |
| --- | --- |
| Primary Ability | Charisma |
| Hit Point Die | D8 per Warlock level |
| Saving Throw | Wisdom and Charisma |
| Proficiencies |  |
| Skill Proficiencies | Choose 2: Arcana, Deception, History, Intimidation, Investigation, Nature, or Religion |
| Weapon Proficiencies | Simple weapons |
| Armor Training | Light armor |
| Starting Equipment | Choose A or B: (A) Leather Armor, Sickle, 2 Daggers, Arcane Focus (orb), Book (occult lore), Scholar’s Pack, and 15 GP; or (B) 100 GP |

**Warlock Features**

| Level | Proficiency Bonus | Class Features | Eldritch Invocations | Cantrips | Prepared Spells | Pact Slots | Pact Slot Level |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | +2 | Eldritch Invocations, Pact Magic | 1 | 2 | 2 | 1 | 1 |
| 2 | +2 | Magical Cunning | 3 | 2 | 3 | 2 | 1 |
| 3 | +2 | Warlock Subclass | 3 | 2 | 4 | 2 | 2 |
| 4 | +2 | Ability Score Improvement | 3 | 3 | 5 | 2 | 2 |
| 5 | +3 | — | 5 | 3 | 6 | 2 | 3 |
| 6 | +3 | Subclass feature | 5 | 3 | 7 | 2 | 3 |
| 7 | +3 | — | 6 | 3 | 8 | 2 | 4 |
| 8 | +3 | Ability Score Improvement | 6 | 3 | 9 | 2 | 4 |
| 9 | +4 | Contact Patron | 7 | 3 | 10 | 2 | 5 |
| 10 | +4 | Subclass feature | 7 | 4 | 10 | 2 | 5 |
| 11 | +4 | Mystic Arcanum (level 6 spell) | 7 | 4 | 11 | 3 | 5 |
| 12 | +4 | Ability Score Improvement | 8 | 4 | 11 | 3 | 5 |
| 13 | +5 | Mystic Arcanum (level 7 spell) | 8 | 4 | 12 | 3 | 5 |
| 14 | +5 | Subclass feature | 8 | 4 | 12 | 3 | 5 |
| 15 | +5 | Mystic Arcanum (level 8 spell) | 9 | 4 | 13 | 3 | 5 |
| 16 | +5 | Ability Score Improvement | 9 | 4 | 13 | 3 | 5 |
| 17 | +6 | Mystic Arcanum (level 9 spell) | 9 | 4 | 14 | 4 | 5 |
| 18 | +6 | — | 10 | 4 | 14 | 4 | 5 |
| 19 | +6 | Epic Boon | 10 | 4 | 15 | 4 | 5 |
| 20 | +6 | Eldritch Master | 10 | 4 | 15 | 4 | 5 |

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/warlock ` |
| ` implementation ` | ` dnd2024:classes_warlock ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` primary_abilities ` | ` ["charisma"] ` |

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
- wisdom
- charisma
skill_proficiencies:
  count: 2
  choices:
  - arcana
  - deception
  - history
  - intimidation
  - investigation
  - nature
  - religion
weapon_proficiencies:
  simple: true
armor_training:
- light
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
- sickle
- dagger
- arcane_focus_varies
- book_25_gp
- scholar_s_pack_40_gp
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
| [Sickle](<../equipment/sickle.md> "dnd2024:equipment/sickle") | ` 1 ` |  |
| [Dagger](<../equipment/dagger.md> "dnd2024:equipment/dagger") | ` 2 ` |  |
| [Arcane Focus (Varies)](<../equipment/arcane_focus_varies.md> "dnd2024:equipment/arcane_focus_varies") | ` 1 ` | ` Orb ` |
| [Book (25 GP)](<../equipment/book_25_gp.md> "dnd2024:equipment/book_25_gp") | ` 1 ` | ` Occult lore ` |
| [Scholar’s Pack (40 GP)](<../equipment/scholar_s_pack_40_gp.md> "dnd2024:equipment/scholar_s_pack_40_gp") | ` 1 ` |  |

###### Currency

<!-- splat:values path=splat_rule/parameters/starting_equipment/options/equipment/currency -->

| Field | Value |
| --- | --- |
| ` gp ` | ` 15 ` |

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
| ` key ` | ` dnd2024:tables/warlock_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- warlock_level_11_mystic_arcanum
- warlock_level_19_epic_boon
- warlock_level_1_eldritch_invocations
- warlock_level_1_pact_magic
- warlock_level_20_eldritch_master
- warlock_level_2_magical_cunning
- warlock_level_3_warlock_subclass
- warlock_level_4_ability_score_improvement
- warlock_level_9_contact_patron
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
| [Level 11: Mystic Arcanum](<../class_features/warlock_level_11_mystic_arcanum.md> "dnd2024:class_features/warlock_level_11_mystic_arcanum") | ` 11 ` |
| [Level 19: Epic Boon](<../class_features/warlock_level_19_epic_boon.md> "dnd2024:class_features/warlock_level_19_epic_boon") | ` 19 ` |
| [Level 1: Eldritch Invocations](<../class_features/warlock_level_1_eldritch_invocations.md> "dnd2024:class_features/warlock_level_1_eldritch_invocations") | ` 1 ` |
| [Level 1: Pact Magic](<../class_features/warlock_level_1_pact_magic.md> "dnd2024:class_features/warlock_level_1_pact_magic") | ` 1 ` |
| [Level 20: Eldritch Master](<../class_features/warlock_level_20_eldritch_master.md> "dnd2024:class_features/warlock_level_20_eldritch_master") | ` 20 ` |
| [Level 2: Magical Cunning](<../class_features/warlock_level_2_magical_cunning.md> "dnd2024:class_features/warlock_level_2_magical_cunning") | ` 2 ` |
| [Level 3: Warlock Subclass](<../class_features/warlock_level_3_warlock_subclass.md> "dnd2024:class_features/warlock_level_3_warlock_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/warlock_level_4_ability_score_improvement.md> "dnd2024:class_features/warlock_level_4_ability_score_improvement") | ` 4 ` |
| [Level 9: Contact Patron](<../class_features/warlock_level_9_contact_patron.md> "dnd2024:class_features/warlock_level_9_contact_patron") | ` 9 ` |
