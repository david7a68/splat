---
splat_schema: 2
splat_id: 0a116960-606b-592f-9315-16325154c317
splat_kind: rule
splat_key: dnd2024:glossary/telepathy
splat_name: Telepathy
splat_origin: dnd2024:reference/glossary/telepathy
splat_sources:
- SRD 5.2.1, pp. 190, https://www.dndbeyond.com/srd
---

# Telepathy

## Source text

<!-- SRD page 190 -->

Telepathy is a magical ability that allows a creature to communicate mentally with another creature within a specified range. Unless a rule states otherwise, the contacted creature doesn’t need to share a language with the telepath to understand this communication, but the contacted creature must be able to understand at least one language or be telepathic itself to understand.

A telepath doesn’t need to see a contacted creature, and the telepath can start or end the telepathic contact at any time (no action required). Telepathic contact can’t be initiated and is immediately broken if either the telepath or the other creature has the Incapacitated condition. Telepathic contact is also broken if the contacted creature is no longer within the telepathy’s range or if the telepath contacts a different creature within range.

A creature without telepathy can receive telepathic messages but can’t initiate a telepathic conversation. Once a telepathic conversation starts, the non-telepath can communicate mentally to the telepath until the telepathic connection ends.

Original layout: [[../../srd-5.2.1/pages/190|page 190]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/telepathy ` |
| ` implementation ` | ` dnd2024:glossary_telepathy ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magical ` | ` true ` |
| ` range ` | ` feature_specified ` |
| ` shared_language_required ` | ` false ` |
| ` recipient_understanding_requires_any ` | ` ["understands_at_least_one_language", "has_telepathy"] ` |
| ` source_rule_can_override_language_requirements ` | ` true ` |
| ` sight_required ` | ` false ` |
| ` initiation_forbidden_if_any_participant_incapacitated ` | ` true ` |
| ` contact_ends_when_any ` | ` ["telepath_incapacitated", "recipient_incapacitated", "recipient_beyond_range", "telepath_contacts_different_creature"] ` |

#### Initiate or end

<!-- splat:values path=splat_rule/parameters/initiate_or_end -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` telepath ` |
| ` timing ` | ` any_time ` |
| ` cost ` | ` none ` |

#### Nontelepath

<!-- splat:values path=splat_rule/parameters/nontelepath -->

| Field | Value |
| --- | --- |
| ` can_receive ` | ` true ` |
| ` can_initiate ` | ` false ` |
| ` can_reply_mentally_during_contact ` | ` true ` |
