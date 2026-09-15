---
splat_schema: 1
splat_id: e6cc51d9-7cce-5e2c-a239-042f26305acb
splat_kind: rule_group
splat_key: dnd2024:species/tiefling
splat_name: Tiefling
splat_origin:
  key: dnd2024:reference/pages/086
splat_sources:
- title: SRD 5.2.1
  locator: pp. 86
  url: https://www.dndbeyond.com/srd
---

# Tiefling

## Source text

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Medium (about 4–7 feet tall) or Small (about 3–4\
feet tall), chosen when you select this species\
Speed: 30 feet\
As a Tiefling, you have the following special traits.\
Darkvision. You have Darkvision with a range of\
60 feet.\
Fiendish Legacy. You are the recipient of a legacy\
that grants you supernatural abilities. Choose a leg-\
acy from the Fiendish Legacies table. You gain the\
level 1 benefit of the chosen legacy.\
When you reach character levels 3 and 5, you\
learn a higher-level spell, as shown on the table.\
You always have that spell prepared. You can cast it\
once without a spell slot, and you regain the ability\
to cast it in that way when you finish a Long Rest.\
You can also cast the spell using any spell slots you\
have of the appropriate level.\
Intelligence, Wisdom, or Charisma is your spell-\
casting ability for the spells you cast with this trait\
(choose the ability when you select the legacy).\
Otherworldly Presence. You know the Thauma-\
turgy cantrip. When you cast it with this trait, the\
spell uses the same spellcasting ability you use for\
your Fiendish Legacy trait.\
Level 3 Level 5\
Ray of Sickness Hold Person\
False Life Ray of Enfeeblement\
Hellish Rebuke Darkness\

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

<!-- splat:yaml path=splat_rule/parameters/size_choice -->

```yaml
count: 1
options:
- size: medium
  height_feet_approximate:
    minimum: 4
    maximum: 7
- size: small
  height_feet_approximate:
    minimum: 3
    maximum: 4
```

#### Fiendish legacy

<!-- splat:yaml path=splat_rule/parameters/fiendish_legacy -->

```yaml
choice:
  count: 1
  options:
  - id: abyssal
    level_1:
      damage_resistance: poison
      cantrip:
        key: dnd2024:spells/poison_spray
    level_3_spell:
      key: dnd2024:spells/ray_of_sickness
    level_5_spell:
      key: dnd2024:spells/hold_person
  - id: chthonic
    level_1:
      damage_resistance: necrotic
      cantrip:
        key: dnd2024:spells/chill_touch
    level_3_spell:
      key: dnd2024:spells/false_life
    level_5_spell:
      key: dnd2024:spells/ray_of_enfeeblement
  - id: infernal
    level_1:
      damage_resistance: fire
      cantrip:
        key: dnd2024:spells/fire_bolt
    level_3_spell:
      key: dnd2024:spells/hellish_rebuke
    level_5_spell:
      key: dnd2024:spells/darkness
level_basis: character_level
spellcasting_ability_choice:
  count: 1
  options:
  - intelligence
  - wisdom
  - charisma
leveled_spell_grants:
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
