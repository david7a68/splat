---
splat_schema: 1
splat_id: ae563e1d-69bc-597d-86fb-9bd7183e2e1a
splat_kind: spell
splat_key: dnd2024:spells/spirit_guardians
splat_name: Spirit Guardians
splat_origin:
  key: dnd2024:reference/spell/spirit_guardians
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164, 165
  url: https://www.dndbeyond.com/srd
---

# Spirit Guardians

## Source text

<!-- SRD page 164 -->

Level 3 Conjuration (Cleric)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a prayer scroll)\
Duration: Concentration, up to 10 minutes

Protective spirits flit around you in a 15-foot Emanation for the duration. If you are good or neutral, their spectral form appears angelic or fey (your choice). If you are evil, they appear fiendish.

When you cast this spell, you can designate creatures to be unaffected by it. Any other creature’s Speed is halved in the Emanation, and whenever the Emanation enters a creature’s space and whenever a creature enters the Emanation or ends its turn there, the creature must make a Wisdom saving throw. On a failed save, the creature takes 3d8 Radiant damage (if you are good or neutral) or 3d8 Necrotic damage (if you are evil). On a successful save, the creature takes half as much damage. A creature makes this save only once per turn.

<!-- SRD page 165 -->

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 3.

Original layout: [[../../srd-5.2.1/pages/164|page 164]], [[../../srd-5.2.1/pages/165|page 165]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a prayer scroll ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/spirit_guardians ` |
| ` implementation ` | ` dnd2024:spell_spirit_guardians ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["area.moves_into_creature", "area.enter", "turn.end"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 15 ` |
| ` origin ` | ` caster ` |

#### Appearance by alignment

<!-- splat:yaml path=splat_rule/parameters/appearance_by_alignment -->

```yaml
good_or_neutral:
  choice:
  - angelic
  - fey
evil: fiendish
```

#### Exempt

<!-- splat:values path=splat_rule/parameters/exempt -->

| Field | Value |
| --- | --- |
| ` designated_creatures ` | ` at_cast ` |

#### Affected

<!-- splat:yaml path=splat_rule/parameters/affected -->

```yaml
speed_fraction_while_inside:
  numerator: 1
  denominator: 2
```

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 3
        sides: 8
        add: 0
    type_by_caster_alignment:
      good_or_neutral: radiant
      evil: necrotic
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` saves_per_creature_per_turn ` | ` 1 ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 3 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 8 ` |
