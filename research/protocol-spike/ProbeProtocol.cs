using System.Buffers.Binary;

namespace Cinis.ProtocolProbe;

// Research-only schema: no HTTP contract, permission field, public financial adapter or floating-point conversion.
internal sealed record FundingRequest(ulong Correlation, ulong Account, ulong Command, byte Asset, ulong Amount)
{
    internal byte[] Encode()
    {
        var body = new byte[35];
        body[0] = body[1] = 1;
        BinaryPrimitives.WriteUInt64BigEndian(body.AsSpan(2), Correlation);
        BinaryPrimitives.WriteUInt64BigEndian(body.AsSpan(10), Account);
        BinaryPrimitives.WriteUInt64BigEndian(body.AsSpan(18), Command);
        body[26] = Asset;
        BinaryPrimitives.WriteUInt64BigEndian(body.AsSpan(27), Amount);
        return body;
    }
}

internal sealed record FundingReply(
    byte Kind,
    byte Code,
    byte Source,
    ulong Sequence,
    uint Events,
    uint Postings,
    ulong BaseAvailable,
    ulong QuoteAvailable)
{
    internal static FundingReply Decode(byte[] body, FundingRequest request)
    {
        if (body.Length != 77 || body[0] != 1 || body[1] != 2)
            throw new InvalidDataException("Unexpected probe schema.");
        if (Number(body, 2) != request.Correlation || Number(body, 10) != request.Account
            || Number(body, 18) != request.Command)
            throw new InvalidDataException("Mismatched correlation/account/command identity.");
        var reply = new FundingReply(body[26], body[27], body[28], Number(body, 29),
            BinaryPrimitives.ReadUInt32BigEndian(body.AsSpan(37)),
            BinaryPrimitives.ReadUInt32BigEndian(body.AsSpan(41)), Number(body, 45), Number(body, 61));
        var valid = reply.Kind switch
        {
            1 => reply.Code is >= 16 and <= 22 && reply.Source == 0 && reply.Sequence == 0,
            2 => reply.Code == 0 && reply.Source is 1 or 2 && reply.Sequence > 0 && reply.Events == 1 && reply.Postings == 1,
            3 => (reply.Code is >= 32 and <= 38 or 63) && reply.Source is 1 or 2 && reply.Sequence > 0,
            4 => reply.Code is >= 1 and <= 5 && reply.Source == 0 && reply.Sequence == 0,
            _ => false
        };
        if (!valid || (reply.Kind != 2 && (reply.Events != 0 || reply.Postings != 0))
            || Number(body, 53) != 0 || Number(body, 69) != 0)
            throw new InvalidDataException("Contradictory funding-probe response.");
        return reply;
    }

    private static ulong Number(byte[] body, int offset) => BinaryPrimitives.ReadUInt64BigEndian(body.AsSpan(offset));
}
