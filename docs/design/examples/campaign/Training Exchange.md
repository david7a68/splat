---
splat_schema: 1
splat_id: d793ec09-562e-4c99-a666-882d2912de02
splat_kind: encounter
splat_key: "test_campaign:encounters/training_exchange"
splat_name: Training Exchange
splat_encounter:
  sides:
    blue: {hostile_to: [red]}
    red: {hostile_to: [blue]}
  roster:
    - key: aster
      creature: {key: "my_homebrew:creatures/guard"}
      side: blue
      controller: {strategy: manual}
      initial: {hp: 30}
    - key: briar
      creature: {key: "my_homebrew:creatures/guard"}
      side: blue
      controller: {strategy: manual}
      initial: {hp: 18}
    - key: cinder
      creature: {key: "my_homebrew:creatures/guard"}
      side: red
      controller: {strategy: manual}
      initial: {hp: 30}
    - key: dusk
      creature: {key: "my_homebrew:creatures/guard"}
      side: blue
      controller: {strategy: manual}
      initial: {hp: 30}
  assumptions:
    - key: reach
      predicate: within_reach
      participants: [aster, briar, cinder, dusk]
      maximum_feet: 30
      value: true
    - key: sight
      predicate: seen
      participants: [aster, briar, cinder, dusk]
      value: true
    - key: consent
      predicate: willing
      participants: [aster, briar, dusk]
      value: true
  completion: {mode: manual}
  limits: {rounds: 20, resolution_events: 10000}
---

A manual design scene, not an unattended strategy. Pairwise reach facts assert
eligibility for all requested ranges up to 30 feet, including touch and 5-foot
reactions. They do not calculate distances. These deliberately permissive facts
are displayed with the run and would be replaced for a real encounter.

Campaign-selected initiative rules still apply. The trace uses specified example
initiative dice to obtain Cinder, Aster, Briar, Dusk in that order.
