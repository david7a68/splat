---
splat_schema: 1
splat_id: 5d029d8a-e7b6-5585-9abe-2c13e96f0527
splat_kind: rule
splat_key: dnd2024:glossary/possession
splat_name: Possession
splat_origin:
  key: dnd2024:reference/glossary/possession
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
---

# Possession

## Source text

<!-- SRD page 186 -->

Some effects cause a creature to be possessed by another creature or entity. A possessing effect defines how the possession operates. Possession can be prevented by the Protection from Evil and Good spell and ended by the Dispel Evil and Good spell.

Original layout: [[../../srd-5.2.1/pages/186|page 186]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/possession ` |
| ` implementation ` | ` dnd2024:glossary_possession ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` operation ` | ` possessing_effect_defined ` |
| ` spell_specific_eligibility_still_applies ` | ` true ` |

#### Prevention source

<!-- splat:values path=splat_rule/parameters/prevention_source -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/protection_from_evil_and_good ` |

#### Ending source

<!-- splat:values path=splat_rule/parameters/ending_source -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dispel_evil_and_good ` |
