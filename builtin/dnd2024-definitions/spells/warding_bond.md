---
splat_schema: 2
splat_id: 4a37cfcf-025d-51f5-9a98-be41338641fe
splat_kind: spell
splat_key: dnd2024:spells/warding_bond
splat_name: Warding Bond
splat_origin: dnd2024:reference/spell/warding_bond
splat_sources:
- SRD 5.2.1, pp. 173, https://www.dndbeyond.com/srd
---

# Warding Bond


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Abjuration |
| Classes | Cleric, Paladin |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a pair of platinum rings worth 50+ GP each, which you and the target must wear for the duration) |
| Duration | 1 hour |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 173 -->

Level 2 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a pair of platinum rings worth 50+ GP each, which you and the target must wear for the duration)\
Duration: 1 hour

You touch another creature that is willing and create a mystic connection between you and the target until the spell ends. While the target is within 60 feet of you, it gains a +1 bonus to AC and saving throws, and it has Resistance to all damage. Also, each time it takes damage, you take the same amount of damage.

The spell ends if you drop to 0 Hit Points or if you and the target become separated by more than 60 feet. It also ends if the spell is cast again on either of the connected creatures.

Original layout: [[../../srd-5.2.1/pages/173|page 173]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/warding_bond ` |
| ` implementation ` | ` dnd2024:spell_warding_bond ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ends_on_any ` | ` ["caster_zero_hp", "separation_exceeds_60_feet", "warding_bond_cast_again_on_either_connected_creature"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` other_than_caster ` | ` true ` |
| ` touch ` | ` true ` |

#### Ongoing requirement

<!-- splat:values path=splat_rule/parameters/ongoing_requirement -->

| Field | Value |
| --- | --- |
| ` caster_and_target_wear_component_rings ` | ` true ` |

#### Benefits

<!-- splat:values path=splat_rule/parameters/benefits -->

| Field | Value |
| --- | --- |
| ` within_caster_feet ` | ` 60 ` |
| ` ac_bonus ` | ` 1 ` |
| ` saving_throw_bonus ` | ` 1 ` |
| ` damage_resistance ` | ` all ` |
| ` duration ` | ` spell_duration ` |

#### Shared damage

<!-- splat:values path=splat_rule/parameters/shared_damage -->

| Field | Value |
| --- | --- |
| ` event ` | ` target.takes_damage ` |
| ` recipient ` | ` caster ` |
| ` amount ` | ` same_as_target_damage_taken ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

