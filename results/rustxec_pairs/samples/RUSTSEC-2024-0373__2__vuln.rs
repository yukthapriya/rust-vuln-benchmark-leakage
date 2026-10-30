fn insert_initial(&mut self, dst_cid: ConnectionId, connection: ConnectionHandle) {
        self.connection_ids_initial
            .insert(dst_cid, RouteDatagramTo::Connection(connection));
    }