---
splat_schema: 2
splat_id: 0670f084-a1af-5192-ad4e-b7804324279a
splat_kind: spell
splat_key: dnd2024:spells/raise_dead
splat_name: Raise Dead
splat_origin: dnd2024:reference/spell/raise_dead
splat_sources:
- SRD 5.2.1, pp. 157, https://www.dndbeyond.com/srd
---

# Raise Dead


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Necromancy |
| Classes | Bard, Cleric, Paladin |
| Casting Time | 1 hour |
| Range |  |
| Components | V, S, M (a diamond worth 500+ GP, which the spell consumes) |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 157 -->

Level 5 Necromancy (Bard, Cleric, Paladin)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (a diamond worth 500+ GP, which the spell consumes)\
Duration: Instantaneous

With a touch, you revive a dead creature if it has been dead no longer than 10 days and it wasn’t Undead when it died.

The creature returns to life with 1 Hit Point. This spell also neutralizes any poisons that affected the creature at the time of death.

This spell closes all mortal wounds, but it doesn’t restore missing body parts. If the creature is lacking body parts or organs integral for its survival— its head, for instance—the spell automatically fails.

Coming back from the dead is an ordeal. The target takes a −4 penalty to D20 Tests. Every time the target finishes a Long Rest, the penalty is reduced by 1 until it becomes 0.

Original layout: [[../../srd-5.2.1/pages/157|page 157]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/raise_dead ` |
| ` implementation ` | ` dnd2024:spell_raise_dead ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` failure_if ` | ` missing_survival_essential_body_parts_or_organs ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` dead_creature ` |
| ` touch ` | ` true ` |
| ` dead_max_days ` | ` 10 ` |
| ` excluded_type_at_death ` | ` undead ` |

#### Revival

<!-- splat:values path=splat_rule/parameters/revival -->

| Field | Value |
| --- | --- |
| ` hit_points ` | ` 1 ` |
| ` neutralizes ` | ` poisons_present_at_death ` |
| ` closes ` | ` all_mortal_wounds ` |
| ` restores_missing_parts ` | ` false ` |

#### Ordeal

<!-- splat:yaml path=splat_rule/parameters/ordeal -->

```yaml
rolls: d20_tests
modifier: -4
recovery:
  event: target.long_rest_finished
  reduction_in_penalty: 1
  stop_at: 0
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

