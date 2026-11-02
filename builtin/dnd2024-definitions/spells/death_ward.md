---
splat_schema: 1
splat_id: b236f1a7-8b7d-57c6-8fb5-d5df6bfb0218
splat_kind: spell
splat_key: dnd2024:spells/death_ward
splat_name: Death Ward
splat_origin:
  key: dnd2024:reference/spell/death_ward
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
---

# Death Ward

## Source text

<!-- SRD page 122 -->

Level 4 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: 8 hours

You touch a creature and grant it a measure of protection from death. The first time the target would drop to 0 Hit Points before the spell ends, the target instead drops to 1 Hit Point, and the spell ends.

If the spell is still in effect when the target is subjected to an effect that would kill it instantly without dealing damage, that effect is negated against the target, and the spell ends.

Original layout: [[../../srd-5.2.1/pages/122|page 122]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "paladin"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/death_ward ` |
| ` implementation ` | ` dnd2024:spell_death_ward ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` touch ` | ` true ` |
| ` count ` | ` 1 ` |

#### Replacements

<!-- splat:table
path: splat_rule/parameters/replacements
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Trigger:
    path: trigger
    type: symbol
  Set hp:
    path: set_hp
    type: integer
    optional: true
  End spell:
    path: end_spell
    type: boolean
  Negate for target:
    path: negate_for_target
    type: boolean
    optional: true
-->

| Key | Trigger | Set hp | End spell | Negate for target |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` First hp drop to zero ` | ` 1 ` | true |  |
| ` row_2 ` | ` Instant death without damage ` |  | true | true |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 8
  unit: hour
```
