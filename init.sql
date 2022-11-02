create database irrigation;
create user 'cosmo'@'localhost' identified with mysql_native_password by 'oldmonk@123';
GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, INDEX, DROP, ALTER, CREATE TEMPORARY TABLES, LOCK TABLES ON irrigation.* TO 'cosmo'@'localhost';
GRANT FILE ON *.* TO 'cosmo'@'localhost';