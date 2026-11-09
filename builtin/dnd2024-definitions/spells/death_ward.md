---
splat_schema: 2
splat_id: b236f1a7-8b7d-57c6-8fb5-d5df6bfb0218
splat_kind: spell
splat_key: dnd2024:spells/death_ward
splat_name: Death Ward
splat_origin: dnd2024:reference/spell/death_ward
splat_sources:
- SRD 5.2.1, pp. 122, https://www.dndbeyond.com/srd
---

# Death Ward


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Abjuration |
| Classes | Cleric, Paladin |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | 8 hours |
| Ritual | No |
| Concentration | No |


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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

