---
splat_schema: 1
splat_id: d96ef08a-2140-5fba-aba1-c9c035989f6f
splat_kind: spell
splat_key: dnd2024:spells/disguise_self
splat_name: Disguise Self
splat_origin:
  key: dnd2024:reference/spell/disguise_self
splat_sources:
- title: SRD 5.2.1
  locator: pp. 124
  url: https://www.dndbeyond.com/srd
---

# Disguise Self

## Source text

<!-- SRD page 124 -->

Level 1 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: 1 hour\
You make yourself—including your clothing, armor,\
weapons, and other belongings on your person—\
look different until the spell ends. You can seem\
1 foot shorter or taller and can appear heavier or\
lighter. You must adopt a form that has the same\
basic arrangement of limbs as you have. Otherwise,\
the extent of the illusion is up to you.\
The changes wrought by this spell fail to hold\
up to physical inspection. For example, if you use\
this spell to add a hat to your outfit, objects pass\
through the hat, and anyone who touches it would\
feel nothing.\
To discern that you are disguised, a creature must\
take the Study action to inspect your appearance\
and succeed on an Intelligence (Investigation) check\
against your spell save DC.\

Original layout: [[../../srd-5.2.1/pages/124|page 124]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/disguise_self ` |
| ` implementation ` | ` dnd2024:spell_disguise_self ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |
| ` includes ` | ` ["clothing", "armor", "weapons", "carried_belongings"] ` |

#### Illusion

<!-- splat:values path=splat_rule/parameters/illusion -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` height_change_max_feet ` | ` 1 ` |
| ` weight_appearance ` | ` ["heavier", "lighter"] ` |
| ` limb_arrangement ` | ` same_basic ` |
| ` remaining_appearance ` | ` caster_choice ` |
| ` physical_inspection ` | ` fails ` |
| ` physical_substance ` | ` false ` |

#### Discern

<!-- splat:yaml path=splat_rule/parameters/discern -->

```yaml
economy: study_action
check:
  ability: intelligence
  skill: investigation
  dc: spell_save_dc
success: disguise_discerned
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: hour
```
