---
splat_schema: 1
splat_id: 5a8679a2-1803-5527-8ffb-ae21f5086537
splat_kind: rule
splat_key: dnd2024:rules/warlock_gift_of_the_protectors
splat_name: Gift of the Protectors
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
---

# Gift of the Protectors

## Source text

<!-- SRD page 73 -->

Prerequisite: Level 9+ Warlock, Pact of the Tome Invocation

A new page appears in your Book of Shadows when you conjure it. With your permission, a creature can take an action to write its name on that page, which can contain a number of names equal to your Charisma modifier (minimum of one name).

When any creature whose name is on the page is reduced to 0 Hit Points but not killed outright, the creature magically drops to 1 Hit Point instead. Once this magic is triggered, no creature can benefit from it until you finish a Long Rest.

As a Magic action, you can erase a name on the page by touching it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_gift_of_the_protectors ` |
| ` implementation ` | ` dnd2024:rule_warlock_gift_of_the_protectors ` |

### Parameters

#### Prerequisite

<!-- splat:values path=splat_rule/parameters/prerequisite -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 9 ` |
| ` invocation ` | ` pact_of_the_tome ` |

#### Book page

<!-- splat:yaml path=splat_rule/parameters/book_page -->

```yaml
created_when: book_of_shadows_conjured
name_capacity:
  op: max
  args:
  - 1
  - variable: charisma_modifier
add_name:
  action_cost: 1
  writer: named_creature
  warlock_permission: true
erase_name:
  action: magic
  touch_page: true
```

#### Protect

<!-- splat:values path=splat_rule/parameters/protect -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` named_creature_reduced_to_zero_hit_points ` |
| ` exclude ` | ` killed_outright ` |
| ` set_hit_points ` | ` 1 ` |
| ` shared_uses ` | ` 1 ` |
| ` recharge ` | ` warlock_long_rest ` |
