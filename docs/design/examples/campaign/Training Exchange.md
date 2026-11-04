---
splat_schema: 2
splat_id: d793ec09-562e-4c99-a666-882d2912de02
splat_kind: encounter
splat_key: "test_campaign:encounters/training_exchange"
splat_name: Training Exchange
splat_sides:
- blue
- red
---

A manual design scene, not an unattended strategy. Pairwise reach facts assert
eligibility for all requested ranges up to 30 feet, including touch and 5-foot
reactions. They do not calculate distances. These deliberately permissive facts
are displayed with the run and would be replaced for a real encounter.

Campaign-selected initiative rules still apply. The trace uses specified example
initiative dice to obtain Cinder, Aster, Briar, Dusk in that order.

## Roster

| Side | Creature | Name | Changes | Note |
| --- | --- | --- | --- | --- |
| blue | [guard](splat:my_homebrew:creatures/guard) |  | Start HP 30 |  %% splat slot=aster %% |
| blue | [guard](splat:my_homebrew:creatures/guard) |  | Start HP 18 |  %% splat slot=briar %% |
| red | [guard](splat:my_homebrew:creatures/guard) |  | Start HP 30 |  %% splat slot=cinder %% |
| blue | [guard](splat:my_homebrew:creatures/guard) |  | Start HP 30 |  %% splat slot=dusk %% |

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
>   slots:
>     aster:
>       controller:
>         strategy: manual
>     briar:
>       controller:
>         strategy: manual
>     cinder:
>       controller:
>         strategy: manual
>     dusk:
>       controller:
>         strategy: manual
>   assumptions:
>   - key: reach
>     predicate: within_reach
>     participants:
>     - aster
>     - briar
>     - cinder
>     - dusk
>     maximum_feet: 30
>     value: true
>   - key: sight
>     predicate: seen
>     participants:
>     - aster
>     - briar
>     - cinder
>     - dusk
>     value: true
>   - key: consent
>     predicate: willing
>     participants:
>     - aster
>     - briar
>     - dusk
>     value: true
> ```

