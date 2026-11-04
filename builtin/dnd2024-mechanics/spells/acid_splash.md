---
splat_schema: 2
splat_id: 7a061d25-9ada-50ee-b2b8-5b7ccfbd063c
splat_kind: spell
splat_key: dnd2024:spells/acid_splash
splat_name: Acid Splash
splat_origin: dnd2024:reference/spell/acid_splash
splat_sources:
- SRD 5.2.1, spell/acid_splash, https://www.dndbeyond.com/srd
---

# Acid Splash


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Evocation |
| Classes |  |
| Casting Time |  |
| Range |  |
| Components | V, S |
| Duration | instantaneous |


Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_actions -->
> 
> ```yaml
> cast:
>   name: Cast Acid Splash
>   activation:
>     economy: action
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
>         min: 0
>         max: 1000
>       allow_self: true
>       range:
>         feet: 60
>       assumption: acid_splash_sphere
>       area:
>         shape: sphere
>         radius_feet: 5
>   costs: []
>   rolls:
>     damage:
>       dice:
>         count:
>           op: add
>           args:
>           - 1
>           - binding: cantrip_tier
>         sides: 6
>         add: 0
>       scope: use
>   steps:
>   - key: resolve
>     operation: dnd2024:save_damage
>     target: chosen
>     ability: dexterity
>     dc:
>       binding: save_dc
>     damage:
>     - type: acid
>       amount:
>         roll: damage
>     success:
>       damage_scale:
>         numerator: 0
>         denominator: 1
> ```
> 
> <!-- splat:yaml path=splat_parameters -->
> 
> ```yaml
> cantrip_tier:
>   type: integer
>   minimum: 0
>   maximum: 3
>   required: true
> save_dc:
>   type: integer
>   minimum: 1
>   maximum: 40
>   required: true
> ```

