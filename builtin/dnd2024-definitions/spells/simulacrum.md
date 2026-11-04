---
splat_schema: 2
splat_id: e263103d-2edf-52c4-a9bd-818245b149c3
splat_kind: spell
splat_key: dnd2024:spells/simulacrum
splat_name: Simulacrum
splat_origin: dnd2024:reference/spell/simulacrum
splat_sources:
- SRD 5.2.1, pp. 162, 163, https://www.dndbeyond.com/srd
---

# Simulacrum


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Illusion |
| Classes | Wizard |
| Casting Time | 12 hours |
| Range |  |
| Components | V, S, M (powdered ruby worth 1,500+ GP, which the spell consumes) |
| Duration | Until dispelled |


## Source text

<!-- SRD page 162 -->

Level 7 Illusion (Wizard)\
Casting Time: 12 hours\
Range: Touch\
Components: V, S, M (powdered ruby worth 1,500+ GP, which the spell consumes)\
Duration: Until dispelled

<!-- SRD page 163 -->

You create a simulacrum of one Beast or Humanoid that is within 10 feet of you for the entire casting of the spell. You finish the casting by touching both the creature and a pile of ice or snow that is the same size as that creature, and the pile turns into the simulacrum, which is a creature. It uses the game statistics of the original creature at the time of casting, except it is a Construct, its Hit Point maximum is half as much, and it can’t cast this spell.

The simulacrum is Friendly to you and creatures you designate. It obeys your commands and acts on your turn in combat. The simulacrum can’t gain levels, and it can’t take Short or Long Rests.

If the simulacrum takes damage, the only way to restore its Hit Points is to repair it as you take a Long Rest, during which you expend components worth 100 GP per Hit Point restored. The simulacrum must stay within 5 feet of you for the repair.

The simulacrum lasts until it drops to 0 Hit Points, at which point it reverts to snow and melts away. If you cast this spell again, any simulacrum you created with this spell is instantly destroyed.

Original layout: [[../../srd-5.2.1/pages/162|page 162]], [[../../srd-5.2.1/pages/163|page 163]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/simulacrum ` |
| ` implementation ` | ` dnd2024:spell_simulacrum ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` types ` | ` ["beast", "humanoid"] ` |
| ` within_caster_feet_for_entire_cast ` | ` 10 ` |
| ` final_touch ` | ` ["target", "same_size_ice_or_snow_pile"] ` |

#### Creation

<!-- splat:yaml path=splat_rule/parameters/creation -->

```yaml
transforms: ice_or_snow_pile
statistics: original_at_casting
replace_type: construct
hp_maximum_fraction:
  numerator: 1
  denominator: 2
cannot_cast: simulacrum
duration: spell_duration
```

#### Behavior

<!-- splat:values path=splat_rule/parameters/behavior -->

| Field | Value |
| --- | --- |
| ` attitude ` | ` friendly ` |
| ` friendly_toward ` | ` caster_and_designated_creatures ` |
| ` obeys ` | ` caster_commands ` |
| ` combat_turn ` | ` caster_turn ` |

#### Restrictions

<!-- splat:values path=splat_rule/parameters/restrictions -->

| Field | Value |
| --- | --- |
| ` gain_levels ` | ` false ` |
| ` short_rest ` | ` false ` |
| ` long_rest ` | ` false ` |

#### Repair

<!-- splat:values path=splat_rule/parameters/repair -->

| Field | Value |
| --- | --- |
| ` only_hp_restoration_method ` | ` true ` |
| ` timing ` | ` caster_long_rest ` |
| ` consumed_component_gp_per_hp ` | ` 100 ` |
| ` distance_to_caster_max_feet ` | ` 5 ` |
| ` throughout_repair ` | ` true ` |

#### Termination

<!-- splat:values path=splat_rule/parameters/termination -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` revert_to_snow_and_melt ` |
| ` caster_recasts_spell ` | ` instantly_destroy_previous_simulacra ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: until_dispelled
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

