-- Add migration script here
-- Drop the existing constraint
ALTER TABLE subscription_tokens DROP CONSTRAINT subscription_tokens_subscriber_id_fkey;

-- Add the new constraint with ON DELETE CASCADE
ALTER TABLE subscription_tokens ADD CONSTRAINT subscription_tokens_subscriber_id_fkey FOREIGN KEY (subscriber_id) REFERENCES subscriptions (id) ON DELETE CASCADE;