---
splat_schema: 2
splat_id: 424a4d39-1fae-5d61-8690-67878afe728d
splat_kind: spell
splat_key: dnd2024:spells/conjure_celestial
splat_name: Conjure Celestial
splat_origin: dnd2024:reference/spell/conjure_celestial
splat_sources:
- SRD 5.2.1, pp. 118, https://www.dndbeyond.com/srd
---

# Conjure Celestial


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Conjuration |
| Classes | Cleric |
| Casting Time | Action |
| Range | 90 feet |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |


## Source text

<!-- SRD page 118 -->

Level 7 Conjuration (Cleric)\
Casting Time: Action\
Range: 90 feet\
Components: V, S\
Duration: Concentration, up to 10 minutes

You conjure a spirit from the Upper Planes, which manifests as a pillar of light in a 10-foot-radius, 40-foot-high Cylinder centered on a point within range. For each creature you can see in the Cylinder, choose which of these lights shines on it:

Healing Light. The target regains Hit Points equal to 4d12 plus your spellcasting ability modifier.

Searing Light. The target makes a Dexterity saving throw, taking 6d12 Radiant damage on a failed save or half as much damage on a successful one.

Until the spell ends, Bright Light fills the Cylinder, and when you move on your turn, you can also move the Cylinder up to 30 feet.

Whenever the Cylinder moves into the space of a creature you can see and whenever a creature you can see enters the Cylinder or ends its turn there, you can bathe it in one of the lights. A creature can be affected by this spell only once per turn.

Using a Higher-Level Spell Slot. The healing and damage increase by 1d12 for each spell slot level above 7.

Original layout: [[../../srd-5.2.1/pages/118|page 118]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/conjure_celestial ` |
| ` implementation ` | ` dnd2024:spell_conjure_celestial ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["spell.cast_completed", "area.moves_into_creature", "area.enter", "turn.end"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cylinder ` |
| ` radius_feet ` | ` 10 ` |
| ` height_feet ` | ` 40 ` |
| ` light ` | ` bright ` |
| ` origin ` | ` point_in_range ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` each_visible_in_area ` |
| ` visible_to_caster ` | ` true ` |

#### Choice per target

<!-- splat:values path=splat_rule/parameters/choice_per_target -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` options ` | ` ["healing_light", "searing_light"] ` |
| ` optional_on_later_triggers ` | ` true ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
healing_light:
  heal:
    dice:
      count: 4
      sides: 12
      add:
        binding: spellcasting_modifier
searing_light:
  save:
    ability: dexterity
    dc:
      binding: save_dc
    failure:
      damage:
        amount:
          dice:
            count: 6
            sides: 12
            add: 0
        type: radiant
    success:
      damage_fraction:
        numerator: 1
        denominator: 2
```

#### Movement

<!-- splat:values path=splat_rule/parameters/movement -->

| Field | Value |
| --- | --- |
| ` event ` | ` caster.moves_on_own_turn ` |
| ` maximum_feet ` | ` 30 ` |

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` effects_per_creature_per_turn ` | ` 1 ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 7 ` |
| ` healing_dice ` | ` 1 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 12 ` |
