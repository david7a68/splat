---
splat_schema: 1
splat_id: 474292db-4ba5-521b-8568-7b91557c05cd
splat_kind: rule
splat_key: dnd2024:glossary/ritual
splat_name: Ritual
splat_origin:
  key: dnd2024:reference/glossary/ritual
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
---

# Ritual

## Source text

<!-- SRD page 187 -->

If you have a spell prepared that has the Ritual tag, you can cast that spell as a Ritual. The Ritual version of a spell takes 10 minutes longer to cast than normal. It also doesn’t expend a spell slot, which means the ritual version of a spell can’t be cast at a higher level. See also “Spells.”

Original layout: [[../../srd-5.2.1/pages/187|page 187]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/ritual ` |
| ` implementation ` | ` dnd2024:glossary_ritual ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` casting_time_added_minutes ` | ` 10 ` |
| ` slot_expended ` | ` false ` |
| ` upcast_allowed ` | ` false ` |

#### Eligibility

<!-- splat:values path=splat_rule/parameters/eligibility -->

| Field | Value |
| --- | --- |
| ` spell_prepared ` | ` true ` |
| ` ritual_tag ` | ` true ` |
