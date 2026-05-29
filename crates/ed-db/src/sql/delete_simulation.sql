DELETE FROM ed_api.simulations
WHERE user_id = $1 AND id = $2;
