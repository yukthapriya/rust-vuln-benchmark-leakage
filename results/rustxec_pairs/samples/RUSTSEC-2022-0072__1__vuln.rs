pub async fn resolve_path(
        &self,
        request_path: &str,
        accept_encoding: AcceptEncoding,
    ) -> Result<ResolveResult<O::File>, IoError> {
        // Sanitize input path.
        let RequestedPath {
            sanitized: mut path,
            is_dir_request,
        } = RequestedPath::resolve(request_path);

        // Try to open the file.
        let file = match self.opener.open(&path).await {
            Ok(pair) => pair,
            Err(err) => return map_open_err(err),
        };

        // The resolved path doesn't contain the trailing slash anymore, so we may
        // have opened a file for a directory request, which we treat as 'not found'.
        if is_dir_request && !file.is_dir {
            return Ok(ResolveResult::NotFound);
        }

        // We may have opened a directory for a file request, in which case we redirect.
        if !is_dir_request && file.is_dir {
            return Ok(ResolveResult::IsDirectory);
        }

        // If not a directory, serve this file.
        if !is_dir_request {
            return self.resolve_final(file, path, accept_encoding).await;
        }

        // Resolve the directory index.
        path.push("index.html");
        let file = match self.opener.open(&path).await {
            Ok(pair) => pair,
            Err(err) => return map_open_err(err),
        };

        // The directory index cannot itself be a directory.
        if file.is_dir {
            return Ok(ResolveResult::NotFound);
        }

        // Serve this file.
        self.resolve_final(file, path, accept_encoding).await
    }