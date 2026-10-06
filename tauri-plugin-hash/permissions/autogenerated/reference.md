## Default Permission

Allow computing SHA-256, SHA-512, and MD5 hashes of strings and files. hash_file reads arbitrary paths the user passes — restrict via capability if you want to limit file access.

#### This default permission set includes the following:

- `allow-hash-string`
- `allow-hash-file`

## Permission Table

<table>
<tr>
<th>Identifier</th>
<th>Description</th>
</tr>


<tr>
<td>

`hash:allow-hash-file`

</td>
<td>

Enables the hash_file command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`hash:deny-hash-file`

</td>
<td>

Denies the hash_file command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`hash:allow-hash-string`

</td>
<td>

Enables the hash_string command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`hash:deny-hash-string`

</td>
<td>

Denies the hash_string command without any pre-configured scope.

</td>
</tr>
</table>
