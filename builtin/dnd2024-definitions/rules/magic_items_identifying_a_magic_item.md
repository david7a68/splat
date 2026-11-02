---
splat_schema: 1
splat_id: 0bf29251-9306-51c3-a8be-2ff8f7bcf1fd
splat_kind: rule
splat_key: dnd2024:rules/magic_items_identifying_a_magic_item
splat_name: Identifying a Magic Item
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102
  url: https://www.dndbeyond.com/srd
---

# Identifying a Magic Item

## Source text

<!-- SRD page 102 -->

Some magic items are indistinguishable from their nonmagical counterparts, while others are conspicuously magical. Handling a magic item is enough to give you a sense that it is extraordinary, but learning a magic item’s properties isn’t automatic. The Identify spell is the fastest way to reveal an item’s properties. Alternatively, you can focus on one magic item during a Short Rest while being in physical contact with the item. At the end of the rest, you learn its properties and how to use them (but not any curse the item might bear).

Sometimes a magic item carries a clue to its properties. The command word to activate a ring might be etched inside the band, or a feathered design might hint that it’s a Ring of Feather Falling.

Wearing or experimenting with an item can also offer hints about its properties. In the specific case of Potions, a little taste is enough to tell the taster what a potion does. Other items might require more experimentation. For example, if your character puts on a Ring of Swimming, the GM might say, “Your movement feels strangely fluid.” Perhaps you then dive into a river to see what happens. The GM would then say you swim unexpectedly well.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_items_identifying_a_magic_item ` |
| ` implementation ` | ` dnd2024:rule_magic_items_identifying_a_magic_item ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` handling_reveals ` | ` extraordinary_nature ` |
| ` automatic_properties_identification ` | ` false ` |

#### Methods

<!-- splat:yaml path=splat_rule/parameters/methods -->

```yaml
identify_spell:
  spell:
    key: dnd2024:spells/identify
short_rest:
  items_maximum: 1
  focus_required: true
  physical_contact_required: true
  reveal_at: rest_end
  reveals:
  - properties
  - usage
  reveals_curse: false
potion_taste:
  quantity: small_taste
  reveals: potion_effect
```

#### Clues and experimentation

<!-- splat:values path=splat_rule/parameters/clues_and_experimentation -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` outputs ` | ` ["property_hints"] ` |
