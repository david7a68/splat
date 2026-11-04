---
splat_schema: 2
splat_id: a95fdbb4-88c5-5f1d-894d-e5f38f436ae0
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_9_contact_patron
splat_name: 'Level 9: Contact Patron'
splat_origin: dnd2024:reference/pages/072
splat_sources:
- SRD 5.2.1, pp. 72, https://www.dndbeyond.com/srd
---

# Level 9: Contact Patron

## Source text

<!-- SRD page 72 -->

In the past, you usually contacted your patron through intermediaries. Now you can communicate directly; you always have the Contact Other Plane spell prepared. With this feature, you can cast the spell without expending a spell slot to contact your patron, and you automatically succeed on the spell’s saving throw.

Once you cast the spell with this feature, you can’t do so in this way again until you finish a Long Rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warlock_level_9_contact_patron ` |
| ` implementation ` | ` dnd2024:class_features_warlock_level_9_contact_patron ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` counts_against_prepared_limit ` | ` false ` |

#### Always prepared

<!-- splat:rows path=splat_rule/parameters/always_prepared -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/contact_other_plane ` |

#### Free cast

<!-- splat:yaml path=splat_rule/parameters/free_cast -->

```yaml
resource:
  id: warlock.contact_patron
  maximum: 1
  recovery:
  - event: rest.long_completed
    amount: all
cost: 1
contact: actor.patron
expend_slot: false
spell_saving_throw: automatic_success
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` minimum_level ` | ` 9 ` |
