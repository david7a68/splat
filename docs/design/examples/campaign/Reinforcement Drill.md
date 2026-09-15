---
splat_schema: 1
splat_id: d793ec09-562e-4c99-a666-882d2912de03
splat_kind: encounter
splat_key: "test_campaign:encounters/reinforcement_drill"
splat_name: Reinforcement Drill
splat_encounter:
  sides:
    blue: {hostile_to: [red]}
    red: {hostile_to: [blue]}
  roster:
    - key: defender
      creature: {key: "my_homebrew:creatures/controller_warden"}
      side: blue
      controller: {strategy: manual}
    - key: reinforcement
      creature: {key: "my_homebrew:creatures/reactive_warden"}
      side: red
      controller: {strategy: manual}
      initial_presence: absent
  assumptions:
    - {key: reach, predicate: within_reach, participants: [defender, reinforcement], maximum_feet: 30, value: true}
    - {key: sight, predicate: seen, participants: [defender, reinforcement], value: true}
  phases:
    - key: arrival
      trigger:
        event: "dnd2024:round.start"
        predicates:
          - predicate: test
            value: {op: eq, args: [{context: event.round}, 3]}
      priority: 0
      repeat: {mode: once}
      required_for_completion: true
      steps:
        - key: arrive
          operation: "splat:participant"
          command: return
          participants: [{roster: reinforcement}]
          reason: scheduled_arrival
  completion:
    mode: objectives
    objectives:
      - key: survive
        when:
          predicate: all
          predicates:
            - {predicate: elapsed_rounds, minimum: 5, boundary: end}
            - {predicate: participant_health, participant: defender, status: alive}
        outcome: {kind: victory, sides: [blue]}
        requires_phases: all_required
        terminal: true
  limits: {rounds: 10, resolution_events: 10000, reaction_depth: 64}
---

A manual schema fixture for an absent actor that enters in round three. The
required arrival prevents an empty opposing field from being treated as an
early completed objective. Manual ending remains available. Its controller can
exercise Frighten, extra reactions, and the declared source-specific rules.
