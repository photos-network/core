-- Production seed: second user, 2 albums, 15 access codes
-- Generated 2026-04-26

BEGIN;

-- ============================================================
-- 1. New account (non-admin)
-- ============================================================
-- Password: FPLG6ydbudvCqw
INSERT INTO accounts (account_id, email, password_hash, display_name, is_admin)
VALUES ('2333012b-71d8-4402-b567-f09808ab1c35', 'albert@photos.network', '$2b$12$r0wWnB1KD994gwzhQ6dtmOIkVfr46s3GgXWwfw2C38Heo99DRlFj.', 'Albert Schütz', FALSE);

-- Available password hashes (bcrypt, cost 12):
-- FPLG6ydbudvCqw => $2b$12$r0wWnB1KD994gwzhQ6dtmOIkVfr46s3GgXWwfw2C38Heo99DRlFj.
-- YakUxmtVdmh73K => $2b$12$dHtSB.0OLIBHLVPkC7cgh.DNosEHkZl11GfZJEtYZYkPadtg8nC5e

-- ============================================================
-- 2. Albums
-- ============================================================
-- Album 1 "Kommunion Fotos"  – owned by existing admin
-- Album 2 "Kommunion Videos" – owned by new user (Albert)
INSERT INTO albums (album_id, owner, name)
VALUES
  ('988bf164-8823-41df-a64d-efe88353ae35', '52e2fd85-766c-44a0-94fb-025a33e48d2f', 'Kommunion Fotos'),
  ('c7589117-898e-4098-be5e-c676a71925cc', '2333012b-71d8-4402-b567-f09808ab1c35', 'Kommunion Videos');

-- ============================================================
-- 3. Album access for Albert (viewer on album 1, owner on album 2)
-- ============================================================
INSERT INTO album_accounts (account_id, album_id, role)
VALUES
  ('2333012b-71d8-4402-b567-f09808ab1c35', '988bf164-8823-41df-a64d-efe88353ae35', 'viewer'),
  ('2333012b-71d8-4402-b567-f09808ab1c35', 'c7589117-898e-4098-be5e-c676a71925cc', 'owner');

-- ============================================================
-- 4. Customers (access codes)
-- ============================================================
-- 13 customers with access to BOTH albums, 2 with access to album 1 only
INSERT INTO customers (customer_id, access_code, display_name)
VALUES
  -- Both albums (1-13)
  ('d769f565-61f2-40b7-ba8a-258b5b5bb5a9', 'XAX9HQGS', 'Aurbacher Barbara'),
  ('678b7e99-823d-4aaa-b01a-2cef6617961e', 'YWPPSL4D', 'Aurbacher Sophia'),
  ('354edac9-500a-4fcf-b97b-225bb265bf78', 'ZW3BKBSB', 'Bernhard Marie'),
  ('e48f8c63-72b1-46d1-ac21-00330c24318b', '5DRH26FQ', 'Dolpp Linus'),
  ('a8358fab-0cc3-428a-beba-d090b75b15fc', 'LNDVDSRF', 'Ernst Fabian'),
  ('d68a9a28-1a01-44b7-8fb3-1f904c0a4351', 'LY6YHB4C', 'Gehring Klara'),
  ('7f271bf3-9aa8-4d50-8742-010e85fb96e4', 'EBT4JCK6', 'Kellner Finn'),
  ('f8c0335c-9974-4497-a83c-c447d115371c', 'NBUW9G2Z', 'Kleele Elias'),
  ('c272d877-1276-48a0-b16e-812afa72c2dd', '25YZQDUB', 'Lempenauer Greta'),
  ('f66dc52d-62d9-403d-af50-36f3dd7c34e1', 'VE9CYPEA', 'Magg Katharina'),
  ('4f665824-c5d2-448c-9ae3-41ef97a27f9c', 'VJFRTX72', 'Prim Charlotte'),
  ('bd6ea147-2fa5-4a60-8334-e4fe08264b02', '39QC2XQN', 'Singer Elias'),
  ('c7392c5b-825c-4034-ade7-193c47aed649', 'C3QL6W7P', 'Vogg Maximilian & Viktoria'),
  ('73bf2c78-e5ac-4ce9-b245-0badc87e0ec8', 'W9QZUE4D', 'Weber Elias'),
  ('ae3d6903-d2f2-45f7-b795-575abd227a25', 'A336DWRA', 'Wilkening Alicia'),
  ('cdba6a63-64d3-4aa9-8589-5aed41ebf797', 'EMRY8RPG', 'Test Access Code');

-- ============================================================
-- 5. Customer ↔ Album assignments
-- ============================================================
-- Customers 1-13: both albums
INSERT INTO customer_albums (customer_id, album_id)
VALUES
  ('d769f565-61f2-40b7-ba8a-258b5b5bb5a9', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('d769f565-61f2-40b7-ba8a-258b5b5bb5a9', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('678b7e99-823d-4aaa-b01a-2cef6617961e', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('678b7e99-823d-4aaa-b01a-2cef6617961e', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('354edac9-500a-4fcf-b97b-225bb265bf78', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('354edac9-500a-4fcf-b97b-225bb265bf78', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('e48f8c63-72b1-46d1-ac21-00330c24318b', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('e48f8c63-72b1-46d1-ac21-00330c24318b', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('a8358fab-0cc3-428a-beba-d090b75b15fc', '988bf164-8823-41df-a64d-efe88353ae35'),
  -- ('a8358fab-0cc3-428a-beba-d090b75b15fc', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('d68a9a28-1a01-44b7-8fb3-1f904c0a4351', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('d68a9a28-1a01-44b7-8fb3-1f904c0a4351', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('7f271bf3-9aa8-4d50-8742-010e85fb96e4', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('7f271bf3-9aa8-4d50-8742-010e85fb96e4', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('f8c0335c-9974-4497-a83c-c447d115371c', '988bf164-8823-41df-a64d-efe88353ae35'),
  -- ('f8c0335c-9974-4497-a83c-c447d115371c', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('c272d877-1276-48a0-b16e-812afa72c2dd', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('c272d877-1276-48a0-b16e-812afa72c2dd', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('f66dc52d-62d9-403d-af50-36f3dd7c34e1', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('f66dc52d-62d9-403d-af50-36f3dd7c34e1', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('4f665824-c5d2-448c-9ae3-41ef97a27f9c', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('4f665824-c5d2-448c-9ae3-41ef97a27f9c', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('bd6ea147-2fa5-4a60-8334-e4fe08264b02', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('bd6ea147-2fa5-4a60-8334-e4fe08264b02', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('c7392c5b-825c-4034-ade7-193c47aed649', '988bf164-8823-41df-a64d-efe88353ae35'),
  -- ('c7392c5b-825c-4034-ade7-193c47aed649', 'c7589117-898e-4098-be5e-c676a71925cc'),
  -- Customers 14-15: album 1 only
  ('73bf2c78-e5ac-4ce9-b245-0badc87e0ec8', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('73bf2c78-e5ac-4ce9-b245-0badc87e0ec8', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('ae3d6903-d2f2-45f7-b795-575abd227a25', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('ae3d6903-d2f2-45f7-b795-575abd227a25', 'c7589117-898e-4098-be5e-c676a71925cc'),

  ('dba6a63-64d3-4aa9-8589-5aed41ebf797', '988bf164-8823-41df-a64d-efe88353ae35'),
  ('dba6a63-64d3-4aa9-8589-5aed41ebf797', 'c7589117-898e-4098-be5e-c676a71925cc');

COMMIT;
