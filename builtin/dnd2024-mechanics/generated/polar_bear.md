---
splat_schema: 2
splat_id: 73aa9e60-ae93-5f97-b620-7ddc21ed2e3b
splat_kind: creature
splat_key: dnd2024:creatures/polar_bear
splat_name: Polar Bear
splat_origin: dnd2024:reference/creature/polar_bear
splat_sources:
- SRD 5.2.1, pp. 359, https://www.dndbeyond.com/srd
---

# Polar Bear


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 42 |
| Initiative | +2 (12) |
| Speed | 40 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 20 | +5 | +5 |
| Dex | 14 | +2 | +2 |
| Con | 16 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 13 | +1 | +1 |
| Cha | 7 | -2 | -2 |


<!-- SRD page 359 -->

Large Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 42 (5d10 + 15)\
Speed 40 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 14 +2 +2 Con 16 +3 +3\
Int 2 −4 −4 Wis 13 +1 +1 Cha 7 −2 −2\
Skills Perception +5, Stealth +4\
Resistances Cold\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 2 (XP 450; PB +2)

### Actions

Multiattack. The bear makes two Rend attacks.

Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 9 (1d8 + 5) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/359|page 359]].

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> rend:
>   name: Rend
>   activation:
>     economy: action
>     category: attack
>     timing: on_turn
>     cost_at: commit
>   targets:
>     chosen:
>       kind: creature
>       count:
>         min: 1
>         max: 1
>       allow_self: false
>       range:
>         feet: 5
>   rolls:
>     attack:
>       expression: 1d20
>       modifier: 7
>       scope: attack
>     damage:
>       expression: 1d8 + 5
>       scope: attack
>   steps:
>   - key: attack
>     operation: dnd2024:attack
>     kind: melee_weapon
>     target: chosen
>     roll: attack
>     on_hit:
>       damage:
>       - type: slashing
>         amount:
>           roll: damage
> multiattack:
>   name: Multiattack
>   activation:
>     economy: action
>     category: attack
>     timing: on_turn
>     cost_at: commit
>   targets:
>     chosen:
>       kind: creature
>       count:
>         min: 1
>         max: 1
>       allow_self: false
>       range:
>         feet: 5
>   rolls:
>     attack:
>       expression: 1d20
>       modifier: 7
>       scope: attack
>     damage:
>       expression: 1d8 + 5
>       scope: attack
>   steps:
>   - key: multiattack
>     operation: splat:repeat
>     count: 2
>     steps:
>     - key: attack
>       operation: dnd2024:attack
>       kind: melee_weapon
>       target: chosen
>       roll: attack
>       on_hit:
>         damage:
>         - type: slashing
>           amount:
>             roll: damage
> ```
> 
> <!-- splat:yaml path=splat_health -->
> 
> ```yaml
> zero_hp: dnd2024:monster
> ```
> 
> <!-- splat:yaml path=splat_traits -->
> 
> ```yaml
> damage_affinities:
>   rule: dnd2024:damage_affinities
>   parameters:
>     resistances:
>     - cold
> ```
> 
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> swim_speed:
>   value: 40
>   unit: ft
> ```

