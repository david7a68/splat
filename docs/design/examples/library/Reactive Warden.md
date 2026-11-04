---
tags:
- homebrew
- splat-design
splat_schema: 2
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e09
splat_kind: creature
splat_key: my_homebrew:creatures/reactive_warden
splat_name: Reactive Warden
splat_languages:
- common
splat_origin: 6b02d7d9-19a7-4a18-a5ee-8bb9f00a04bd
---

# Reactive Warden


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Humanoid |
| Armor Class | 16 |
| Hit Points | 30 |
| Initiative | = {"op":"ability_modifier","args":[{"stat":{"of":"actor","key":"dexterity"}}]} |
| Speed | 30 |
| Challenge |  |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 |  |
| Dex | 14 | +2 |  |
| Con | 14 | +2 |  |
| Int | 12 | +1 |  |
| Wis | 12 | +1 |  |
| Cha | 10 | +0 |  |


A homebrew test creature. Its package definition never stores damage, spent
charges, or other state from a run.

**Strike.** `splat:use strike` · `splat:roll strike.attack`

**Mend.** `splat:use mend` · `splat:roll mend.healing`

**Protect Ally.** `splat:use protect` is enabled only at a valid reaction opportunity.

**Ember Wave.** `splat:use ember_wave`

<!-- splat:inventory -->

| Key | Item | Quantity | Equipment | Attuned |
| --- | --- | --- | --- | --- |
| armor | `my_homebrew:items/scale_mail` | 1 | worn | false |
| spear | `my_homebrew:items/spear` | 1 | wielded | false |
| shield | `my_homebrew:items/shield` | 1 | carried | false |
| potions | `my_homebrew:items/healing_draught` | 3 | carried | false |
| wand_a | `my_homebrew:items/dual_wand` | 1 | carried | false |
| wand_b | `my_homebrew:items/dual_wand` | 1 | carried | false |
| pack | `my_homebrew:containers/supply_pack` | 1 | worn | false |

The supply pack supplies its own contents. There is no local `container=pack`
table to merge with it. Item controls are rendered with the selected inventory
instance as their source; a carried wand must be wielded before use.

This independent homebrew copy adds a head-dependent, opportunity-attack-only
reaction budget. Its capacity refresh uses the agreed next-turn ruling. It does
not give those uses to Protect Ally. Ordinary opportunity attacks come from the
selected core rule, not from an extra whole-action use of Strike.

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> strike:
>   name: Strike
>   activation:
>     economy: action
>     category: attack
>     timing: on_turn
>     cost_at: commit
>   requirements:
>   - predicate: equipment_state
>     row: spear
>     state: wielded
>   targets:
>     chosen:
>       kind: creature
>       count:
>         min: 1
>         max: 1
>       range:
>         feet: 5
>   rolls:
>     attack:
>       expression: 1d20
>       modifier: 5
>       scope: attack
>     damage:
>       expression: 1d8 + 3
>       scope: attack
>   steps:
>   - key: attack
>     operation: dnd2024:attack
>     target: chosen
>     kind: melee_weapon
>     weapon_row: spear
>     roll: attack
>     on_hit:
>       damage:
>       - type: piercing
>         amount:
>           roll: damage
> mend:
>   name: Mend
>   activation:
>     economy: action
>     category: feature
>     timing: on_turn
>     cost_at: commit
>   targets:
>     chosen:
>       kind: creature
>       count:
>         min: 1
>         max: 1
>       allow_self: true
>       willing: true
>       range:
>         touch: true
>   costs:
>   - from: actor
>     resource: healing
>     amount: 1
>     at: commit
>   rolls:
>     healing:
>       expression: 1d8 + 2
>       scope: target
>   steps:
>   - key: healing
>     operation: dnd2024:heal
>     target: chosen
>     amount:
>       roll: healing
> protect:
>   name: Protect Ally
>   activation:
>     economy: reaction
>     category: feature
>     cost_at: commit
>     trigger:
>       event: dnd2024:attack.hit_pending
>       predicates:
>       - predicate: relation
>         subject: event.target
>         to: actor
>         value: ally
>       - predicate: different_actor
>         left: event.target
>         right: actor
>       - predicate: within_reach
>         from: actor
>         to: event.target
>         feet: 5
>   effects:
>     ward:
>       stacking:
>         key: my_homebrew:protect_ally
>         policy: dnd2024:same_effect
>       duration:
>         until: resolution_end
>         of: event.resolution
>       modifiers:
>       - stat: armor_class
>         operation: add
>         value: 2
>         scope: event.resolution
>   steps:
>   - key: ward
>     operation: splat:apply_effect
>     target:
>       context: event.target
>     effect: ward
> ember_wave:
>   name: Ember Wave
>   use:
>     entry:
>       key: my_homebrew:spells/ember_wave
>     action: cast
>     bindings:
>       slot_pool: spell_slots_1
>       cast_level: 1
>       save_dc: 13
> ```
> 
> <!-- splat:yaml path=splat_budgets -->
> 
> ```yaml
> extra_opportunities:
>   name: Reactive Heads
>   kind: reaction
>   capacity:
>     op: max
>     args:
>     - 0
>     - op: subtract
>       args:
>       - stat:
>           of: actor
>           key: heads
>       - 1
>   eligible:
>     predicate: test
>     value:
>       op: eq
>       args:
>       - context: use.category
>       - literal: opportunity_attack
>   refresh:
>     event: dnd2024:turn.start
>     subject: actor
>   capacity_at: refresh
> ```
> 
> <!-- splat:yaml path=splat_hit_dice -->
> 
> ```yaml
> count: 4
> die: d8
> ```
> 
> <!-- splat:yaml path=splat_inventory_details -->
> 
> ```yaml
> wand_a:
>   resources:
>     fire: 4
>     healing: 2
> wand_b:
>   resources:
>     fire: 1
>     healing: 1
> ```
> 
> <!-- splat:yaml path=splat_proficiencies -->
> 
> ```yaml
> saves:
> - strength
> - constitution
> skills:
>   athletics: proficient
>   perception: proficient
> armor:
> - light
> - medium
> - shield
> weapons:
> - simple
> ```
> 
> <!-- splat:yaml path=splat_resources -->
> 
> ```yaml
> healing:
>   name: Healing Uses
>   max: 3
>   recovery:
>   - key: long_rest
>     mechanic: dnd2024:long_rest
>     restore: all
> spell_slots_1:
>   name: Level 1 Spell Slots
>   kind: spell_slot
>   level: 1
>   max: 2
>   recovery:
>   - key: long_rest
>     mechanic: dnd2024:long_rest
>     restore: all
> ```
> 
> <!-- splat:yaml path=splat_senses -->
> 
> ```yaml
> passive_perception: 13
> ```
> 
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> heads:
>   value: 4
> armor_class:
>   rebuild: dnd2024:armor_class
>   includes:
>   - inventory.armor.base
>   - ability.dexterity
> proficiency_bonus:
>   value: 2
> ```

