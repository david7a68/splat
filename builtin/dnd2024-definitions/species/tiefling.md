---
splat_schema: 2
splat_id: e6cc51d9-7cce-5e2c-a239-042f26305acb
splat_kind: rule_group
splat_key: dnd2024:species/tiefling
splat_name: Tiefling
splat_origin: dnd2024:reference/pages/086
splat_sources:
- SRD 5.2.1, pp. 86, https://www.dndbeyond.com/srd
---

# Tiefling

## Source text

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Medium (about 4–7 feet tall) or Small (about 3–4 feet tall), chosen when you select this species\
Speed: 30 feet

As a Tiefling, you have the following special traits.

Darkvision. You have Darkvision with a range of 60 feet.

Fiendish Legacy. You are the recipient of a legacy that grants you supernatural abilities. Choose a legacy from the Fiendish Legacies table. You gain the level 1 benefit of the chosen legacy.

When you reach character levels 3 and 5, you learn a higher-level spell, as shown on the table. You always have that spell prepared. You can cast it once without a spell slot, and you regain the ability to cast it in that way when you finish a Long Rest. You can also cast the spell using any spell slots you have of the appropriate level.

Intelligence, Wisdom, or Charisma is your spellcasting ability for the spells you cast with this trait (choose the ability when you select the legacy).

Otherworldly Presence. You know the Thaumaturgy cantrip. When you cast it with this trait, the spell uses the same spellcasting ability you use for your Fiendish Legacy trait.

**Fiendish Legacies**

| Legacy | Level 1 | Level 3 | Level 5 |
| --- | --- | --- | --- |
| Abyssal | You have Resistance to Poison damage. You also know the Poison Spray cantrip. | Ray of Sickness | Hold Person |
| Chthonic | You have Resistance to Necrotic damage. You also know the Chill Touch cantrip. | False Life | Ray of Enfeeblement |
| Infernal | You have Resistance to Fire damage. You also know the Fire Bolt cantrip. | Hellish Rebuke | Darkness |

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/tiefling ` |
| ` implementation ` | ` dnd2024:species_tiefling ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` speed_feet ` | ` 30 ` |
| ` darkvision_feet ` | ` 60 ` |

#### Size choice

<!-- splat:values path=splat_rule/parameters/size_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/size_choice/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Size:
    path: size
    type: symbol
  Height feet approximate minimum:
    path: height_feet_approximate/minimum
    type: integer
  Height feet approximate maximum:
    path: height_feet_approximate/maximum
    type: integer
-->

| Key | Size | Height feet approximate minimum | Height feet approximate maximum |
| --- | --- | --- | --- |
| ` row_1 ` | ` Medium ` | ` 4 ` | ` 7 ` |
| ` row_2 ` | ` Small ` | ` 3 ` | ` 4 ` |

#### Fiendish legacy

<!-- splat:values path=splat_rule/parameters/fiendish_legacy -->

| Field | Value |
| --- | --- |
| ` level_basis ` | ` character_level ` |

##### Choice

<!-- splat:values path=splat_rule/parameters/fiendish_legacy/choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |

###### Options

<!-- splat:table
path: splat_rule/parameters/fiendish_legacy/choice/options
key: Variant
order:
- abyssal
- chthonic
- infernal
columns:
  Variant:
    path: id
    type: symbol
  Level 1 damage resistance:
    path: level_1/damage_resistance
    type: symbol
  Level 1 cantrip:
    path: level_1/cantrip
    type: reference
  Level 3 spell:
    path: level_3_spell
    type: reference
  Level 5 spell:
    path: level_5_spell
    type: reference
-->

| Variant | Level 1 damage resistance | Level 1 cantrip | Level 3 spell | Level 5 spell |
| --- | --- | --- | --- | --- |
| ` Abyssal ` | ` Poison ` | [Poison Spray](<../spells/poison_spray.md> "dnd2024:spells/poison_spray") | [Ray of Sickness](<../spells/ray_of_sickness.md> "dnd2024:spells/ray_of_sickness") | [Hold Person](<../spells/hold_person.md> "dnd2024:spells/hold_person") |
| ` Chthonic ` | ` Necrotic ` | [Chill Touch](<../spells/chill_touch.md> "dnd2024:spells/chill_touch") | [False Life](<../spells/false_life.md> "dnd2024:spells/false_life") | [Ray of Enfeeblement](<../spells/ray_of_enfeeblement.md> "dnd2024:spells/ray_of_enfeeblement") |
| ` Infernal ` | ` Fire ` | [Fire Bolt](<../spells/fire_bolt.md> "dnd2024:spells/fire_bolt") | [Hellish Rebuke](<../spells/hellish_rebuke.md> "dnd2024:spells/hellish_rebuke") | [Darkness](<../spells/darkness.md> "dnd2024:spells/darkness") |

##### Spellcasting ability choice

<!-- splat:values path=splat_rule/parameters/fiendish_legacy/spellcasting_ability_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` options ` | ` ["intelligence", "wisdom", "charisma"] ` |

##### Leveled spell grants

<!-- splat:yaml path=splat_rule/parameters/fiendish_legacy/leveled_spell_grants -->

```yaml
always_prepared: true
each_spell_free_cast:
  spell_slot_cost: 0
  uses:
    maximum: 1
    recover:
      event: rest.long_rest_completed
      amount: all
can_use_appropriate_spell_slots: true
```

#### Otherworldly presence

<!-- splat:yaml path=splat_rule/parameters/otherworldly_presence -->

```yaml
cantrip:
  key: dnd2024:spells/thaumaturgy
ability: chosen_fiendish_legacy_spellcasting_ability
```
