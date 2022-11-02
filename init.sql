create database irrigation;
create user 'cosmo'@'localhost' identified with mysql_native_password by 'oldmonk@123';
GRANT SELECT, INSERT, UPDATE, DELETE, CREATE, INDEX, DROP, ALTER, CREATE TEMPORARY TABLES, LOCK TABLES ON irrigation.* TO 'cosmo'@'localhost';
GRANT FILE ON *.* TO 'cosmo'@'localhost';

use irrigation;


create table irrigation(
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `sensor_name` varchar(500) not null,
  `capacity` int(11) not null,
  `timestamp` datetime default current_timestamp,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `deleted` tinyint(1) DEFAULT '0',
  PRIMARY KEY (`id`)
);

CREATE TABLE `preference` (
  `id` int NOT NULL AUTO_INCREMENT,
  `min_irrigation_interval_in_minutes` int NOT NULL,
  `irrigation_time_in_seconds` int NOT NULL,
  `capacity_buffer` int NOT NULL,
  `sensor_name` varchar(500) NOT NULL,
  `signal_pin` int NOT NULL,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `deleted` tinyint(1) DEFAULT '0',
  PRIMARY KEY (`id`)
);

create table daily_measurement(
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `sensor_name` varchar(500) not null,
  `capacity` int(11) not null,
  `timestamp` datetime default current_timestamp,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `deleted` tinyint(1) DEFAULT '0',
  PRIMARY KEY (`id`)
);

create table hourly_measurement(
	`id` int(11) NOT NULL AUTO_INCREMENT,
    `sensor_name` varchar(500) not null,
    `capacity` int(11) not null,
    `timestamp` datetime default current_timestamp,
   `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `deleted` tinyint(1) DEFAULT '0',
  PRIMARY KEY (`id`)
);

create table minutely_measurement(
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `sensor_name` varchar(500) not null,
  `capacity` int(11) not null,
  `timestamp` datetime default current_timestamp,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `deleted` tinyint(1) DEFAULT '0',
  PRIMARY KEY (`id`)
);

create table secondly_measurement(
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `sensor_name` varchar(500) not null,
  `capacity` int(11) not null,
  `timestamp` datetime default current_timestamp,
  `created_at` datetime DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `deleted` tinyint(1) DEFAULT '0',
  PRIMARY KEY (`id`)
);