---
splat_schema: 2
splat_id: d793ec09-562e-4c99-a666-882d2912de03
splat_kind: encounter
splat_key: "test_campaign:encounters/reinforcement_drill"
splat_name: Reinforcement Drill
splat_rounds: 10
splat_sides:
- blue
- red
---

A manual schema fixture for an absent actor that enters in round three. The
required arrival prevents an empty opposing field from being treated as an
early completed objective. Manual ending remains available. Its controller can
exercise Frighten, extra reactions, and the declared source-specific rules.

## Roster

| Side | Creature | Name | Changes | Note |
| --- | --- | --- | --- | --- |
| blue | [controller warden](splat:my_homebrew:creatures/controller_warden) |  |  |  %% splat slot=defender %% |
| red | [reactive warden](splat:my_homebrew:creatures/reactive_warden) |  |  |  %% splat slot=reinforcement %% |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_list_data -->
> 
> ```yaml
> splat_encounter:
>   sides:
>     blue:
>       hostile_to:
>       - red
>     red:
>       hostile_to:
>       - blue
>   limits:
>     reaction_depth: 64
>   assumptions:
>   - key: reach
>     predicate: within_reach
>     participants:
>     - defender
>     - reinforcement
>     maximum_feet: 30
>     value: true
>   - key: sight
>     predicate: seen
>     participants:
>     - defender
>     - reinforcement
>     value: true
>   phases:
>   - key: arrival
>     trigger:
>       event: dnd2024:round.start
>       predicates:
>       - predicate: test
>         value:
>           op: eq
>           args:
>           - context: event.round
>           - 3
>     priority: 0
>     repeat:
>       mode: once
>     required_for_completion: true
>     steps:
>     - key: arrive
>       operation: splat:participant
>       command: return
>       participants:
>       - roster: reinforcement
>       reason: scheduled_arrival
>   completion:
>     mode: objectives
>     objectives:
>     - key: survive
>       when:
>         predicate: all
>         predicates:
>         - predicate: elapsed_rounds
>           minimum: 5
>           boundary: end
>         - predicate: participant_health
>           participant: defender
>           status: alive
>       outcome:
>         kind: victory
>         sides:
>         - blue
>       requires_phases: all_required
>       terminal: true
>   slots:
>     defender:
>       controller:
>         strategy: manual
>     reinforcement:
>       initial_presence: absent
>       controller:
>         strategy: manual
> ```

