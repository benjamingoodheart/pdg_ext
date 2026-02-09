# Introduction
This library is functions that will make PDG's life easier when it comes to creating various queries in their day to day work.



# Examples

### Int Arrays

```sql
select dedupe_int_array('{1,2,3,3}')  
```
*returns:*
```bash
dedupe_int_array 
------------------
{2,3,1}
(1 row)
```
### Float Arrays

```sql
 select dedupe_float_array('{1.0, 2.1, 3.3, 4.5, 3.3, 2.1}'); 
```
*returns:*
```bash
dedupe_float_array 
--------------------
{3.3,2.1,4.5,1}
(1 row)
```


### String Arrays
```sql
select dedupe_string_array('{"hello", "yo", "hello", "yo", "ben"}');

```
*returns:*
```bash
dedupe_string_array 
---------------------
{hello,yo,ben}
(1 row)
```