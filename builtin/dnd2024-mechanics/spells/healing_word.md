---
splat_schema: 2
splat_id: 2108cd7b-cda7-54b0-94f1-220ff3ac75cc
splat_kind: spell
splat_key: dnd2024:spells/healing_word
splat_name: Healing Word
splat_origin: dnd2024:reference/spell/healing_word
splat_sources:
- SRD 5.2.1, spell/healing_word, https://www.dndbeyond.com/srd
---

# Healing Word


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Abjuration |
| Classes |  |
| Casting Time |  |
| Range |  |
| Components | V |
| Duration | instantaneous |


Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> cast:
>   name: Cast Healing Word
>   activation:
>     economy: bonus_action
>     category: magic
>     timing: on_turn
>     cost_at: commit
>   requirements:
>   - predicate: dnd2024:spell_casting
>     spell: self
>   targets:
>     chosen:
>       kind: creature
>       count:
>         min: 1
>         max: 1
>       allow_self: true
>       range:
>         feet: 60
>       visibility: seen
>   costs:
>   - from: actor
>     resource:
>       binding: slot_pool
>     amount: 1
>     at: dnd2024:spell_slot_expenditure
>   rolls:
>     healing:
>       dice:
>         count:
>           op: multiply
>           args:
>           - 2
>           - binding: cast_level
>         sides: 4
>         add:
>           binding: spellcasting_modifier
>       scope: target
>   steps:
>   - key: resolve
>     operation: dnd2024:heal
>     target: chosen
>     amount:
>       roll: healing
> ```
> 
> <!-- splat:yaml path=splat_parameters -->
> 
> ```yaml
> cast_level:
>   type: integer
>   minimum: 1
>   maximum: 9
>   required: true
> slot_pool:
>   type: actor_resource
>   required: true
> spellcasting_modifier:
>   type: integer
>   minimum: -5
>   maximum: 10
>   required: true
> ```

