## Default Permission

Allow reading the system hostname and the current username. Both commands are read-only, perform no I/O, and have no side effects — safe to enable by default. NOTE: the hostname is a low-sensitivity identifier but it does reveal machine identity; restrict via capability if you don't want JS to see it.

#### This default permission set includes the following:

- `allow-hostname`
- `allow-username`

## Permission Table

<table>
<tr>
<th>Identifier</th>
<th>Description</th>
</tr>


<tr>
<td>

`hostname:allow-hostname`

</td>
<td>

Enables the hostname command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`hostname:deny-hostname`

</td>
<td>

Denies the hostname command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`hostname:allow-username`

</td>
<td>

Enables the username command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`hostname:deny-username`

</td>
<td>

Denies the username command without any pre-configured scope.

</td>
</tr>
</table>
