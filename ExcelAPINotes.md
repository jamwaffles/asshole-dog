# Requests

## Get used range to find insert position

<https://graph.microsoft.com/v1.0/me/drive/items/43DF93283C30B7A4!90283/workbook/worksheets/Dev/usedrange>

```json
// The last populated row - add one to this to get the next insert position
{ "rowCount": 200 }
```

## Insert range

Insert two values; one a date, the other a 1/0 flag

<https://graph.microsoft.com/v1.0/me/drive/items/43DF93283C30B7A4!90283/workbook/worksheets/Dev/range(address='A1:B1')>

```json
{
  "values": [
    [
      "2019-09-06 18:30:00",
      1
    ]
  ],
  "numberFormat": [
    [
      "dd/mm/yyyy hh:mm",
      "0"
    ]
  ]
}
```

# Column formats

- Timestamp (value type `Double`): `dd/mm/yyyy hh:mm` (see [Timestamps](#timestamps) for format info)
- Flags (value type `Double`): `0` (`bool` as number)

# Timestamps

- They accept dates like `2019/01/02 03:04:05` or `2019-09-06 18:30:00` but not stuff like RFC3339
- Timestamps are [stored as the decimal number of days since 1900](http://www.cpearson.com/excel/datetime.htm)