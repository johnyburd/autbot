ALTER TABLE IF EXISTS public.tb_auto_responses ADD COLUMN IF NOT EXISTS reply BOOLEAN NOT NULL DEFAULT '0';