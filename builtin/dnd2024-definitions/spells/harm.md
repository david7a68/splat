---
splat_schema: 1
splat_id: 87cc2289-5542-5c7f-ab89-d16e578280ee
splat_kind: spell
splat_key: dnd2024:spells/harm
splat_name: Harm
splat_origin:
  key: dnd2024:reference/spell/harm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
---

# Harm

## Source text

<!-- SRD page 139 -->

Level 6 Necromancy (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous

You unleash virulent magic on a creature you can see within range. The target makes a Constitution saving throw. On a failed save, it takes 14d6 Necrotic damage, and its Hit Point maximum is reduced by an amount equal to the Necrotic damage it took. On a successful save, it takes half as much damage only. This spell can’t reduce a target’s Hit Point maximum below 1.

Original layout: [[../../srd-5.2.1/pages/139|page 139]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` necromancy ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/harm ` |
| ` implementation ` | ` dnd2024:spell_harm ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
failure:
  damage:
    dice: 14d6
    type: necrotic
  hp_maximum_reduction:
    amount: necrotic_damage_taken
    minimum_resulting_hp_maximum: 1
success:
  damage_fraction:
    numerator: 1
    denominator: 2
  hp_maximum_reduction: false
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
