---
splat_schema: 1
splat_id: d793ec09-562e-4c99-a666-882d2912de04
splat_kind: sequence
splat_key: "test_campaign:sequences/two_exchanges"
splat_name: Two Exchanges
splat_sequence:
  steps:
    - key: first
      encounter: {key: "test_campaign:encounters/training_exchange"}
      bindings: {aster: aster, briar: briar, cinder: cinder, dusk: dusk}
    - key: pause
      elapsed: {seconds: 60}
    - key: rest
      rest:
        mechanic: "dnd2024:short_rest"
        participants: [aster, briar, cinder, dusk]
        duration_seconds: 3600
    - key: second
      encounter: {key: "test_campaign:encounters/training_exchange"}
      bindings: {aster: aster, briar: briar, cinder: cinder, dusk: dusk}
---

Bindings map encounter roster keys to persistent sequence member keys. The
first binding creates each member from its roster definition. The second uses
the same participants and current state, after an explicitly completed rest.
It does not reset Briar to the first encounter's authored 18 HP.

Short Rest choices such as spending Hit Dice remain controller choices. A dead
or otherwise ineligible participant does not gain rest benefits merely because
its key is listed. This fixture is manual; it is not an unattended simulation.
