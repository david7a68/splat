---
splat_schema: 2
splat_id: bcfa9253-12b1-553a-bc7c-eaf05c6eb8ec
splat_kind: spell
splat_key: dnd2024:spells/conjure_elemental
splat_name: Conjure Elemental
splat_origin: dnd2024:reference/spell/conjure_elemental
splat_sources:
- SRD 5.2.1, pp. 118, https://www.dndbeyond.com/srd
---

# Conjure Elemental


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Conjuration |
| Classes | Druid, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 118 -->

Level 5 Conjuration (Druid, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

You conjure a Large, intangible spirit from the Elemental Planes that appears in an unoccupied space within range. Choose the spirit’s element, which determines its damage type: air (Lightning), earth (Thunder), fire (Fire), or water (Cold). The spirit lasts for the duration.

Whenever a creature you can see enters the spirit’s space or starts its turn within 5 feet of the spirit, you can force that creature to make a Dexterity saving throw if the spirit has no creature Restrained. On failed save, the target takes 8d8 damage of the spirit’s type, and the target has the Restrained condition until the spell ends. At the start of each of its turns, the Restrained target repeats the save. On a failed save, the target takes 4d8 damage of the spirit’s type. On a successful save, the target isn’t Restrained by the spirit.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 5.

Original layout: [[../../srd-5.2.1/pages/118|page 118]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/conjure_elemental ` |
| ` implementation ` | ` dnd2024:spell_conjure_elemental ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` location ` |
| ` within_range ` | ` true ` |
| ` unoccupied ` | ` true ` |

#### Spirit

<!-- splat:yaml path=splat_rule/parameters/spirit -->

```yaml
size: large
intangible: true
duration: spell_duration
element_damage_choice:
  air: lightning
  earth: thunder
  fire: fire
  water: cold
```

#### Capture

<!-- splat:yaml path=splat_rule/parameters/capture -->

```yaml
events:
- creature.enters_spirit_space
- creature.starts_turn_within_5_feet
target_visible_to_caster: true
optional: true
requires: spirit_has_no_restrained_creature
save:
  ability: dexterity
  failure:
    damage:
      dice: 8d8
      type: chosen_element_damage
    condition: restrained
    until: spell_ends_or_repeat_save_success
```

#### Restrained repeat

<!-- splat:yaml path=splat_rule/parameters/restrained_repeat -->

```yaml
event: target.turn_started
save:
  ability: dexterity
  failure:
    damage:
      dice: 4d8
      type: chosen_element_damage
  success:
    remove: spirit_restrained
```

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 5 ` |
| ` capture_damage_per_level ` | ` 1d8 ` |
| ` repeat_damage_per_level ` | ` 1d8 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 10
  unit: minute
```
