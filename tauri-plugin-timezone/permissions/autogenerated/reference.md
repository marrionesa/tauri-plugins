## Default Permission

Allow reading the system IANA timezone and the current UTC offset. Both commands are read-only, perform no I/O, and have no side effects — safe to enable by default.

#### This default permission set includes the following:

- `allow-get-timezone`
- `allow-get-utc-offset-seconds`

## Permission Table

<table>
<tr>
<th>Identifier</th>
<th>Description</th>
</tr>


<tr>
<td>

`timezone:allow-get-timezone`

</td>
<td>

Enables the get_timezone command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`timezone:deny-get-timezone`

</td>
<td>

Denies the get_timezone command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`timezone:allow-get-utc-offset-seconds`

</td>
<td>

Enables the get_utc_offset_seconds command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`timezone:deny-get-utc-offset-seconds`

</td>
<td>

Denies the get_utc_offset_seconds command without any pre-configured scope.

</td>
</tr>
</table>
