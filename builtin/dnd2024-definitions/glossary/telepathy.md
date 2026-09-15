---
splat_schema: 1
splat_id: 0a116960-606b-592f-9315-16325154c317
splat_kind: rule
splat_key: dnd2024:glossary/telepathy
splat_name: Telepathy
splat_origin:
  key: dnd2024:reference/glossary/telepathy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/telepathy
  parameters:
    magical: true
    range: feature_specified
    shared_language_required: false
    recipient_understanding_requires_any:
    - understands_at_least_one_language
    - has_telepathy
    source_rule_can_override_language_requirements: true
    sight_required: false
    initiate_or_end:
      chooser: telepath
      timing: any_time
      cost: none
    initiation_forbidden_if_any_participant_incapacitated: true
    contact_ends_when_any:
    - telepath_incapacitated
    - recipient_incapacitated
    - recipient_beyond_range
    - telepath_contacts_different_creature
    nontelepath:
      can_receive: true
      can_initiate: false
      can_reply_mentally_during_contact: true
  implementation: dnd2024:glossary_telepathy
splat_contracts:
- id: dnd2024:glossary_telepathy
  inputs:
    telepath:
      type: participant
      required: true
    recipient:
      type: participant
      required: true
    range_feet:
      type: number
      required: true
    operation:
      type: choice
      required: true
  outputs:
  - contact_state
  - communication_eligibility
  events:
  - telepathy.contact_requested
  - condition.changed
  - movement.completed
  - telepathy.contact_ended
  capabilities:
  - communication
  - relationships
  - conditions
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 5f14a117d81409e311b5de2bc363be72d9e454574d19266c80ec3a1e1345bb7d
  source_path: builtin/srd-5.2.1/glossary/telepathy.md
  pages:
  - 190
  engine_gaps: &id001
  - dnd2024:glossary_telepathy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: b56dc6bfd6882d85f8e0a73f1343a1dad0d749fa33a466bb477b3365830be3d6
splat_clauses:
- key: source_rules
  text: '# Telepathy Telepathy is a magical ability that allows a creature to communicate mentally with
    another creature within a specified range. Unless a rule states otherwise, the contacted creature
    doesn’t need to share a language with the telepath to understand this communication, but the contacted
    creature must be able to understand at least one language or be telepathic itself to understand. A
    telepath doesn’t need to see a contacted creature, and the telepath can start or end the telepathic
    contact at any time (no action required). Telepathic contact can’t be initiated and is immediately
    broken if either the telepath or the other creature has the Incapacitated condition. Telepathic contact
    is also broken if the contacted creature is no longer within the telepathy’s range or if the telepath
    contacts a different creature within range. A creature without telepathy can receive telepathic messages
    but can’t initiate a telepathic conversation. Once a telepathic conversation starts, the non-telepath
    can communicate mentally to the telepath until the telepathic connection ends.'
  sha256: 73b0bed790998b3af1b09981d9a94dfbb3287bb4690063ecdc5a08e102f9312e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Telepathy

<!-- SRD page 190 -->

Telepathy is a magical ability that allows a creature\
to communicate mentally with another creature\
within a specified range. Unless a rule states other-\
wise, the contacted creature doesn’t need to share a\
language with the telepath to understand this com-\
munication, but the contacted creature must be able\
to understand at least one language or be telepathic\
itself to understand.\
A telepath doesn’t need to see a contacted crea-\
ture, and the telepath can start or end the telepathic\
contact at any time (no action required). Telepathic\
contact can’t be initiated and is immediately broken\
if either the telepath or the other creature has the\
Incapacitated condition. Telepathic contact is also\
broken if the contacted creature is no longer within\
the telepathy’s range or if the telepath contacts a\
different creature within range.\
A creature without telepathy can receive tele-\
pathic messages but can’t initiate a telepathic con-\
versation. Once a telepathic conversation starts,\
the non-telepath can communicate mentally to the\
telepath until the telepathic connection ends.\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].
