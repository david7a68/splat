---
splat_schema: 1
splat_id: b9a97dc5-72f3-50aa-9b9b-4d952a58ad86
splat_kind: spell
splat_key: dnd2024:spells/antilife_shell
splat_name: Antilife Shell
splat_origin:
  key: dnd2024:reference/spell/antilife_shell
splat_sources:
- title: SRD 5.2.1
  locator: pp. 109
  url: https://www.dndbeyond.com/srd
---

# Antilife Shell

## Source text

<!-- SRD page 109 -->

Level 5 Abjuration (Druid)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 1 hour

An aura extends from you in a 10-foot Emanation for the duration. The aura prevents creatures other than Constructs and Undead from passing or reaching through it. An affected creature can cast spells or make attacks with Ranged or Reach weapons through the barrier.

If you move so that an affected creature is forced to pass through the barrier, the spell ends.

Original layout: [[../../srd-5.2.1/pages/109|page 109]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["druid"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
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
| ` mechanic ` | ` dnd2024:spells/antilife_shell ` |
| ` implementation ` | ` dnd2024:spell_antilife_shell ` |

### Parameters

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 10 ` |
| ` origin ` | ` caster ` |
| ` duration ` | ` spell_duration ` |

#### Barrier

<!-- splat:values path=splat_rule/parameters/barrier -->

| Field | Value |
| --- | --- |
| ` excluded_types ` | ` ["construct", "undead"] ` |
| ` blocks ` | ` ["creature_passing", "creature_reaching"] ` |
| ` allows ` | ` ["spell_casting_through", "ranged_weapon_attacks_through", "reach_weapon_attacks_through"] ` |

#### End when

<!-- splat:values path=splat_rule/parameters/end_when -->

| Field | Value |
| --- | --- |
| ` caster_moves_forcing_affected_creature_through ` | ` true ` |
